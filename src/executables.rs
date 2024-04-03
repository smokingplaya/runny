use yaml_rust::Yaml;
use crate::{logger, runner};

fn bad_yml(yml: &Yaml, key: &str) -> bool {
    let val = &yml[key];

    val.is_null() || val.is_badvalue()
}

fn err(err: &str) {
    logger::error(&format!("failed to parse settings.yml. Error: {}", err));
}

#[allow(dead_code)]
pub fn preset(_yml: &Yaml) {
    
}

pub fn script(yml: &Yaml) {
    let runner: String = if bad_yml(yml, "runner-default") {String::from("current")} else {String::from(yml["runner-default"].as_str().unwrap())};

    runner::set(runner);

    if bad_yml(yml, "presets") {
        return err("field \"presets\" is empty!");
    }

    let presets = yml["presets"].as_hash();

    match presets {
        Some(data) => {
            println!("{:?}", data);
        },
        None => err("field \"presets\" is empty!"),
    }
}