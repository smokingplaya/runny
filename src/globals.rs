/*
    Rust be like:
*/

use std::{collections::HashMap, sync::Mutex};

use lazy_static::lazy_static;

lazy_static! {
    static ref GLOBALS: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

#[allow(dead_code)]
pub fn set(name: &str, value: String) {
    let mut globals = GLOBALS.lock().unwrap();
    globals.insert(name.to_string(), value);
}

#[allow(dead_code)]
pub fn get(name: &str) -> String {
    String::from(GLOBALS.lock().unwrap().get(name).unwrap())
}