/*
    Rust be like:
*/

use std::{collections::HashMap, sync::Mutex};

use lazy_static::lazy_static;
use yaml_rust::Yaml;

lazy_static! {
    static ref GLOBALS: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
    static ref GLOBAL_YAML: Mutex<Option<Yaml>> = Mutex::new(None);
}

pub fn set(name: &str, value: String) {
    let mut globals = GLOBALS.lock().unwrap();
    globals.insert(name.to_string(), value);
}

pub fn get(name: &str) -> String {
    String::from(GLOBALS.lock().unwrap().get(name).unwrap())
}

// ну да, костыль, и че

pub fn set_yaml(value: Yaml) {
    let mut data = GLOBAL_YAML.lock().unwrap();
    *data = Some(value);
}

pub fn get_yaml() -> Yaml {
    GLOBAL_YAML.lock().unwrap().clone().unwrap()
}