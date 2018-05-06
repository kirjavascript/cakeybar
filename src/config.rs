extern crate toml;

use std::fs::File;
use std::io::prelude::*;
use std::process::exit;

/// This is what we're going to decode into. Each field is optional, meaning
/// that it doesn't have to be present in TOML.
#[derive(Debug, Deserialize)]
pub struct Config {
    global_string: Option<String>,
    global_integer: Option<u64>,
    server: Option<ServerConfig>,
    peers: Option<Vec<PeerConfig>>,
}

/// Sub-structs are decoded from tables, so this will decode from the `[server]`
/// table.
///
/// Again, each field is optional, meaning they don't have to be present.
#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    ip: Option<String>,
    port: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct PeerConfig {
    ip: Option<String>,
    port: Option<u64>,
}

pub fn parse_config(filename: &str) {
    let mut file = File::open(filename);

    if file.is_err() {
        println!("config file {} not found", filename);
        exit(1i32);
    }

    let mut contents = String::new();
    file.unwrap()
        .read_to_string(&mut contents)
        .expect("something went wrong reading the config");


    let decoded: Config = toml::from_str(&contents).unwrap();
    println!("{:#?}", decoded);
}
