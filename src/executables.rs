use yaml_rust::Yaml;
use crate::{globals, logger};

fn bad_yml(yml: &Yaml, key: &str) -> bool {
    let val = &yml[key];

    val.is_null() || val.is_badvalue()
}

fn err(err: &str) {
    logger::error(&format!("failed to parse settings.yml. Error: {}", err));
}

#[allow(dead_code)]
fn preset(_yml: &Yaml) {
}

pub fn script(yml: &Yaml) {
    let runner: String = if bad_yml(yml, "runner-default") {String::from("current")} else {String::from(yml["runner-default"].as_str().unwrap())};

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