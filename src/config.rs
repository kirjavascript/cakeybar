extern crate toml;

use std::fs::File;
use std::io::prelude::*;
use std::process::exit;

#[derive(Debug, Deserialize)]
pub struct Config {
    theme: Option<String>,
    bar: Vec<BarConfig>,
}

#[derive(Debug, Deserialize)]
pub struct BarConfig {
    monitor: Option<u64>,
    position: Option<String>,
    class: Option<String>,
}

pub fn parse_config(filename: &str) -> Config {
    let mut file_result = File::open(filename);

    if let Err(e) = file_result {
        eprintln!("{}: {}", filename, e);
        exit(2i32);
    }

    let mut contents = String::new();
    file_result.unwrap()
        .read_to_string(&mut contents)
        .expect("something went wrong reading the config");

    let decoded_result: Result<Config, _> = toml::from_str(&contents);

    if let Err(e) = decoded_result {
        eprintln!("{}: {}", filename, e);
        exit(1i32);
    }

    let config = decoded_result.unwrap();

    config
}
