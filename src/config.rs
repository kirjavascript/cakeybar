extern crate toml;

use std::fs::File;
use std::io::prelude::*;
use std::process::exit;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub bar: Vec<BarConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub enum Position {
    top,
    bottom,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BarConfig {
    name: Option<String>,
    monitor: Option<u64>,
    position: Option<Position>,
    theme: Option<String>,
}

impl BarConfig {
    pub fn get_monitor_index(&self) -> usize {
        self.monitor.unwrap_or(0) as usize
    }
    pub fn get_position(&self) -> Position {
        self.position.clone().unwrap_or(Position::top)
    }
}

pub fn parse_config(filename: &str) -> Config {
    let file_result = File::open(filename);

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

    // TODO: assert unique names for modules

    let config = decoded_result.unwrap();

    config
}
