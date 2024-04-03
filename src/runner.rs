/*
    Rust be like:
*/

use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref RUNNER: Mutex<String> = Mutex::new(String::new());
}

#[allow(dead_code)]
pub fn set(name: String) {
    let mut runner = RUNNER.lock().unwrap();
    *runner = name;
}

#[allow(dead_code)]
pub fn get() -> String {
    let runner = RUNNER.lock().unwrap();
    runner.clone()
}