mod symbols;
mod label_group;

pub use self::symbols::SymbolFmt;
pub use self::label_group::LabelGroup;

use std::env;
use std::process::Command;

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
