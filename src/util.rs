use super::gdk::{Screen, ScreenExt, Rectangle};
use super::gtk::{CssProvider, CssProviderExt, StyleContext};

use std::process::Command;
use std::{thread, str};

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
        Err(e) => println!("Error parsing stylesheet:\n{}", e),
    };
}

pub fn run_command(exec: String) {
    thread::spawn(enclose!(exec move || {
        let exec_clone = exec.clone();
        let split: Vec<&str> = exec_clone.split(" ").collect();
        let output = Command::new(split.get(0).unwrap_or(&""))
            .args(&split[1..])
            .output();
        match output {
            Ok(out) => {
                let stderr = str::from_utf8(&out.stderr).unwrap_or("");
                if stderr != "" {
                    eprintln!("{}", stderr);
                }
            },
            Err(err) => {
                eprintln!("{}: {}", err, exec);
            },
        }
    }));
}
