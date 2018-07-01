use gdk::{Screen, ScreenExt, Rectangle};
use gtk::{CssProvider, CssProviderExt, StyleContext};

pub fn get_monitors() -> Vec<Rectangle> {
    let screen = Screen::get_default().unwrap();
    let mut monitors = Vec::new();
    for i in 0..screen.get_n_monitors() {
        monitors.push(screen.get_monitor_geometry(i));
    }
    monitors
}

pub fn get_dimensions() -> (i32, i32) {
    let (width, height) = (Screen::width(), Screen::height());
    (width, height)
}

pub fn show_monitor_debug() {
    super::gtk::init().ok();
    let (width, height) = get_dimensions();
    println!("Screen: {}x{}", width, height);
    let monitors = get_monitors();
    for (i, mon) in monitors.iter().enumerate() {
        let &Rectangle { x, y, width, height } = mon;
        println!("Monitor {}: {}x{} x: {} y: {}", i, width, height, x, y);
    }
}

pub fn load_theme(path: &str) {
    let screen = Screen::get_default().unwrap();
    let provider = CssProvider::new();
    match provider.load_from_path(path) {
        Ok(_) => StyleContext::add_provider_for_screen(&screen, &provider, 0),
        Err(e) => {error!("parsing stylesheet:\n{}", e);},
    };
}

use std::env;

pub fn get_config_dir() -> String {
    if let Ok(xdg_path) = env::var("XDG_CONFIG_HOME") {
        format!("{}/{}", xdg_path, ::NAME)
    } else if let Ok(home_path) = env::var("HOME") {
        format!("{}/.config/{}", home_path, ::NAME)
    } else {
        format!("~/.config/{}", ::NAME)
    }
}

use std::process::Command;
use std::{thread, str};

pub fn run_command(exec: String) {
    thread::spawn(clone!(exec move || {
        let split: Vec<&str> = exec.split(" ").collect();
        let output = Command::new(split.get(0).unwrap_or(&""))
            .args(&split[1..])
            .output();
        match output {
            Ok(out) => {
                let stderr = str::from_utf8(&out.stderr).unwrap_or("");
                if stderr != "" {
                    warn!("{}", stderr);
                }
            },
            Err(err) => {
                warn!("{}: {}", err, exec);
            },
        }
    }));
}
