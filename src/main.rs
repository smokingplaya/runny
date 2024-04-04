/*
    * Runny
    * A small utility, that helps with run commands

    ! smokingplaya feat 𝖬𝖠𝖦𝖠 𝖲𝖮𝖫𝖴𝖳𝖨𝖮𝖭𝖲™
*/

mod logger;
mod finder;
mod executables;
mod globals;

use std::{env, fs::File, io::Read};
use yaml_rust::YamlLoader;

fn handle(mut file: File) {
    let mut content: String = String::new();
    let _ = file.read_to_string(&mut content);

    match YamlLoader::load_from_str(&content) {
        Ok(config) => executables::script(&config[0]),
        Err(err) => logger::error(&format!("settings.yml contains invalid syntax!\n\tScanError: {}", err.to_string()))
    }
}

fn main() {
    // currently useless code
    
    let argv: Vec<String> = env::args().collect();
    let argc: i32 = argv.len() as i32;

    let preset: String = String::from(if argc == 1 { "default" } else { argv.get(2).unwrap() }); // yep

    globals::set("preset", preset);

    match finder::settings() {
        Ok(file) => handle(file),
        Err(_) => return
    };
}