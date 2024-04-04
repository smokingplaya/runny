use std::process::{Command, Stdio};

use yaml_rust::Yaml;
use crate::{globals, logger};

/*
    other yaml functions
*/

fn is_bad(yml: &Yaml) -> bool {
    yml.is_null() || yml.is_badvalue()
}

fn is_bad_yml(yml: &Yaml, key: &str) -> bool {
    is_bad(&yml[key])
}

fn is_yml_list(yml: &Yaml) -> bool {
    match yml.as_vec() {
        Some(_) => true,
        None => false
    }
}

fn yml_run(yml: &Yaml, key: &str) {
    let field = &yml[key];
    let field_is_list = is_yml_list(&field);

    let runner = globals::get("runner");

    if field_is_list {
        for item in field.as_vec().unwrap() {
            command(runner.clone(), item.as_str().unwrap());
        }
    } else {
        preset(&globals::get_yaml()[field.as_str().unwrap()]);
    }
}

/*
    other
*/

fn err(err: &str) {
    logger::error(&format!("failed to parse settings.yml. Error: {}", err));
}

fn get_argument(shell: &str) -> &str {
    match shell {
        "cmd" => "/C",
        "powershell" => "-Command",
        "bash" => "-c",
        "zsh" => "-c",

        _ => ""
    }
}

fn command(runner: String, command: &str) {
    let arg = get_argument(&runner);

    if arg.is_empty() {
        return logger::error("runny don't support your shell. :(");
    }

    let res = Command::new(runner.clone())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .args([arg, command])
        .output();

    match res {
        Ok(_) => {},
        Err(err) => logger::error(&err.to_string()),
    }
}

fn preset(yml: &Yaml) {
    let runner = if is_bad_yml(yml, "runner") {globals::get("runner")} else {String::from(yml["runner"].as_str().unwrap())};

    if is_bad_yml(yml, "command") {
        return logger::error("failed to find \"command\" field!");
    }

    let cmd = &yml["command"];
    let command_is_list = is_yml_list(cmd);

    let run_field = &yml["run"];
    let has_run_field = !is_bad(run_field);
    let has_run_before_field = if has_run_field {!is_bad_yml(run_field, "before")} else {false};
    let has_run_after_field = if has_run_field {!is_bad_yml(run_field, "after")} else {false};

    // run before

    if has_run_field && has_run_before_field {
        yml_run(run_field, "before");
    }

    // run command

    if command_is_list {
        for item in cmd.as_vec().unwrap() {
            command(runner.clone(), item.as_str().unwrap());
        }
    } else {
        command(runner, cmd.as_str().unwrap());
    }

    // run after

    if has_run_field && has_run_after_field {
        yml_run(run_field, "after");
    }
}

pub fn script(yml: &Yaml) {
    if is_bad_yml(yml, "runner-default") {
        return logger::error("you don't specify \nrunner-default\n parameter!");
    }

    let runner: String = String::from(yml["runner-default"].as_str().unwrap()).to_lowercase();

    globals::set("runner", runner);

    if is_bad_yml(yml, "presets") {
        return err("field \"presets\" is empty!");
    }

    let presets = &yml["presets"];
    let presets_hash = presets.as_hash();

    globals::set_yaml(presets.clone());
    
    let current_preset = globals::get("preset");

    match presets_hash {
        Some(data) => {
            for (k, _) in data {
                let name = k.as_str().unwrap();

                if is_bad_yml(&presets, name) {
                    err(&format!("field presets.{} is broken!", name));
                    break;
                }

                if current_preset != name {
                    continue;
                }

                let current_preset = &presets[name];

                preset(current_preset);
            }
        },
        None => err("field \"presets\" is broken!"),
    }
}