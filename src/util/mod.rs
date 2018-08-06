mod symbols;
mod label_group;
mod format_bytes;

pub use self::symbols::format_symbols;
pub use self::label_group::LabelGroup;
pub use self::format_bytes::format_bytes;

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
