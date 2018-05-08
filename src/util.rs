use super::gdk::{Screen, ScreenExt, Rectangle};
use super::gtk::{CssProvider, CssProviderExt, StyleContext};

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
    super::gtk::init().ok(); // ok to ensure result is used
    let (width, height) = get_dimensions();
    println!("Screen: {}x{}", width, height);
    let monitors = get_monitors();
    for (i, mon) in monitors.iter().enumerate() {
        let &Rectangle { x, y, width, height } = mon;
        println!("Monitor {}: {}x{} x: {} y: {}", i, width, height, x, y);
    }
}

pub fn load_theme(src: &str) {
    let screen = Screen::get_default().unwrap();
    let provider = CssProvider::new();
    match provider.load_from_data(src.as_bytes()) {
        Ok(_) => {
            StyleContext::add_provider_for_screen(&screen, &provider, 0);
        },
        Err(_) => println!("Error parsing stylesheet"),
    };
}
