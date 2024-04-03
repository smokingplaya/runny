/*
    rust-logger v.0.0.1
        coded by smokingplaya<3 2024
*/

enum LoggerLevel {
    Info,
    Warning,
    Error
}

impl LoggerLevel {
    fn as_string(&self) -> &str {
        match *self {
            LoggerLevel::Info => "info",
            LoggerLevel::Warning => "warning",
            LoggerLevel::Error => "error",
        }
    }
}

fn print(level: LoggerLevel, message: &str) {
    println!("[runny][{}] {}", level.as_string(), message);
}

#[allow(dead_code)]
pub fn info(message: &str) {
    print(LoggerLevel::Info, message);
}

#[allow(dead_code)]
pub fn warning(message: &str) {
    print(LoggerLevel::Warning, message);
}

#[allow(dead_code)]
pub fn error(message: &str) {
    print(LoggerLevel::Error, message);
}