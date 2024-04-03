/*
    * Runny
    * A small utility, that helps with run commands

    ! smokingplaya feat 𝖬𝖠𝖦𝖠 𝖲𝖮𝖫𝖴𝖳𝖨𝖮𝖭𝖲™
*/

mod logger;
mod finder;
mod executables;
mod runner;

use std::{fs::File, io::Read};
use yaml_rust::YamlLoader;

fn runny(mut file: File) {
    let mut content: String = String::new();
    let _ = file.read_to_string(&mut content);

    match YamlLoader::load_from_str(&content) {
        Ok(config) => executables::script(&config[0]),
        Err(err) => logger::error(&format!("settings.yml contains invalid syntax!\n\tScanError: {}", err.to_string()))
    }
}

fn main() {
    // currently useless code
    
    //let argv: Vec<String> = env::args().collect();
    //let _argc: i32 = argv.len() as i32;

    match finder::settings() {
        Ok(file) => runny(file),
        Err(_) => return
    };
}