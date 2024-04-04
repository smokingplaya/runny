use std::process::{Command, Stdio};

use yaml_rust::Yaml;
use crate::{globals, logger};

fn bad_yml(yml: &Yaml, key: &str) -> bool {
    let val = &yml[key];

    val.is_null() || val.is_badvalue()
}

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

fn command(runner: String, command: String) {
    let arg = get_argument(&runner);

    if arg.is_empty() {
        return logger::error("runny don't support your shell. :(");
    }

    let res = Command::new(runner.clone())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .args([arg, &command])
        .output();

    match res {
        Ok(_) => {},
        Err(err) => logger::error(&err.to_string()),
    }
}

fn preset(yml: &Yaml) {
    let runner = if bad_yml(yml, "runner") {globals::get("runner")} else {String::from(yml["runner"].as_str().unwrap())};

    if bad_yml(yml, "command") {
        return logger::error("failed to find \"command\" field!");
    }

    let command_is_list: bool =  match &yml["command"].as_vec() {
        Some(_) => true,
        None => false
    };

    if command_is_list {
        return logger::info("it's list!");
    }

    command(runner, (&yml["command"].as_str().unwrap()).to_string());
}

// парсит settings.yml и вызывает presets
pub fn script(yml: &Yaml) {
    if bad_yml(yml, "runner-default") {
        return logger::error("you don't specify \nrunner-default\n parameter!");
    }

    let runner: String = String::from(yml["runner-default"].as_str().unwrap()).to_lowercase();

    globals::set("runner", runner);

    if bad_yml(yml, "presets") {
        return err("field \"presets\" is empty!");
    }

    let presets = &yml["presets"];
    let presets_hash = presets.as_hash();

    let current_preset = globals::get("preset");

    match presets_hash {
        Some(data) => {
            for (k, _) in data {
                let name = k.as_str().unwrap();

                if bad_yml(&presets, name) {
                    err(&format!("field presets.{} is broken!", name));
                    break;
                }

                if current_preset != name {
                    continue;
                }

                preset(&presets[name]);
            }
        },
        None => err("field \"presets\" is broken!"),
    }
}