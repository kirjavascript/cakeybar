use gdk::{Display, DisplayExt, MonitorExt, Screen};
use glib::Error;
use gtk::{CssProvider, CssProviderExt, Rectangle, StyleContext};

mod window;
pub use self::window::*;

pub fn css_reset() {
    let screen = Screen::get_default().unwrap();
    let provider = CssProvider::new();
    if provider.load_from_data(include_bytes!("reset.css")).is_ok() {
        StyleContext::add_provider_for_screen(
            &screen,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}

pub fn load_theme(path: &str) -> Result<CssProvider, Error> {
    let screen = Screen::get_default().unwrap();
    let provider = CssProvider::new();
    match provider.load_from_path(path) {
        Ok(_) => {
            StyleContext::add_provider_for_screen(
                &screen,
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
            Ok(provider)
        }
        Err(e) => Err(e)
    }
}

pub fn unload_theme(provider: &CssProvider) {
    let screen = Screen::get_default().unwrap();
    StyleContext::remove_provider_for_screen(&screen, provider);
}

// monitor stuff

pub fn get_monitor_geometry() -> Vec<Rectangle> {
    let display = Display::get_default().unwrap();
    let mut monitors = Vec::new();
    for i in 0..display.get_n_monitors() {
        let monitor = display.get_monitor(i).unwrap();
        monitors.push(monitor.get_geometry())
    }
    monitors
}

pub fn get_monitor_name(monitor_index: i32) -> Option<String> {
    let display = Display::get_default()?;
    let monitor = display.get_monitor(monitor_index)?;
    monitor.get_model()
}

pub fn show_monitor_debug() {
    gtk::init().ok();
    let display = Display::get_default().unwrap();
    for i in 0..display.get_n_monitors() {
        if let Some(monitor) = display.get_monitor(i) {
            let geometry = monitor.get_geometry();
            let Rectangle {
                x,
                y,
                width,
                height,
            } = geometry;
            let model = monitor.get_model().unwrap_or("".to_string());
            println!(
                "Monitor {}: {} @ {}x{} x: {} y: {}",
                i, model, width, height, x, y
            );
        }
    }
}
