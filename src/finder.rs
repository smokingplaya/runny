use std::{env, fs::File, path::Path};
use crate::logger;

pub fn settings() -> Result<File, String> {
    let cwd_buf = env::current_dir().unwrap();
    let cwd = cwd_buf.display().to_string();

    let runny_path = format!("{}\\.runny", cwd);

    if !Path::new(&runny_path).exists() {
        logger::error(".runny folder doesn't exist!");

        return Err(String::from(""));
    }

    let runny_file_path = format!("{}\\settings.yml", runny_path);

    if !Path::new(&runny_file_path).exists() {
        logger::error("settings.yml file doesn't exist!");

        return Err(String::from(""));
    }

    Ok(File::open(runny_file_path).unwrap())
}