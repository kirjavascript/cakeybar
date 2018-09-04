mod symbols;
mod label_group;
mod timer;

pub use self::symbols::SymbolFmt;
pub use self::label_group::LabelGroup;
pub use self::timer::Timer;

use std::env;

use std::process::Command;

use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::path::Path;

pub fn get_config_dir() -> String {
    if let Ok(xdg_path) = env::var("XDG_CONFIG_HOME") {
        format!("{}/{}", xdg_path, ::NAME)
    } else if let Ok(home_path) = env::var("HOME") {
        format!("{}/.config/{}", home_path, ::NAME)
    } else {
        format!("~/.config/{}", ::NAME)
    }
}

pub fn run_command(command: String) {
    Command::new("/bin/sh")
        .arg("-c")
        .arg(command)
        .spawn()
        .ok();
}


pub fn read_file(path: &str) -> Result<String, Error> {
    let path = Path::new(path);
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.trim().to_string())
}

pub fn format_bytes(bytes: u64) -> String {
    if bytes == 0 {
        return "0B".to_string();
    }
    const LEN: usize = 5;
    let bytes = bytes as f64;
    let sizes: [&str; LEN] = ["", "K", "M", "G", "T"];
    let index = ((bytes).ln() / 1024_f64.ln()).floor();
    let val = bytes / (1024_f64.powf(index));
    let index = index as usize;
    let suffix = if index < LEN { sizes[index] } else { "?" };
    format!("{:.*}{}B", if index < 2 { 0 } else { 2 }, val, suffix)
}
