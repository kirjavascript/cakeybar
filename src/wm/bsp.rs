use std::process::Command;
use std::thread;

pub fn set_padding(is_top: bool, padding: i32) {
    let position = if is_top { "top" } else { "bottom" };

    thread::spawn(move || {
        Command::new("bspc")
            .arg("config")
            .arg(format!("{}_padding", position))
            .arg(format!("{}", padding))
            .output()
            .ok();
    });
}
