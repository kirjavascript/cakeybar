mod label_group;
mod symbols;
mod timer;

pub use self::label_group::LabelGroup;
pub use self::symbols::SymbolFmt;
pub use self::timer::Timer;

use std::process::Command;

use std::fs::{self, File};
use std::io::prelude::*;
use std::io::Error;
use std::path::Path;

pub fn run_command(command: String) {
    Command::new("/bin/sh").arg("-c").arg(command).spawn().ok();
}

pub fn read_file(path: &str) -> Result<String, Error> {
    let path = Path::new(path);
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.trim().to_string())
}

pub fn write_file(path: &str, data: &str) -> Result<(), Error> {
    let path = Path::new(path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut file = File::create(path)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}

pub fn format_bytes(bytes: u64) -> String {
    if bytes == 0 {
        return "0B".to_string()
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
