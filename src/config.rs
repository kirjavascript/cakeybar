extern crate toml;

use std::fs::File;
use std::io::prelude::*;
use std::process::exit;

#[derive(Debug, Deserialize)]
pub struct Config {
    theme: Option<String>,
    bar: Option<Vec<BarConfig>>,
}

#[derive(Debug, Deserialize)]
pub struct BarConfig {
    monitor: Option<u64>,
    position: Option<String>,
}

pub fn parse_config(filename: &str) -> Config {
    let mut file_result = File::open(filename);

    if file_result.is_err() {
        eprintln!("{}: {}", filename, file_result.err().unwrap());
        exit(2i32);
    }

    let mut contents = String::new();
    file_result.unwrap()
        .read_to_string(&mut contents)
        .expect("something went wrong reading the config");

    let decoded_result: Result<Config, _> = toml::from_str(&contents);

    if decoded_result.is_err() {
        eprintln!("{}: {}", filename, decoded_result.err().unwrap());
        exit(1i32);
    }

    decoded_result.unwrap()
}
