use gtk;
use gtk::{
    Rectangle,
    CssProvider,
    CssProviderExt,
    StyleContext,
};
use gdk::{
    Screen,
    Display,
    DisplayExt,
    MonitorExt,
};

mod window;
pub use self::window::*;

pub fn load_theme(path: &str) {
    let screen = Screen::get_default().unwrap();
    let provider = CssProvider::new();
    match provider.load_from_path(path) {
        Ok(_) => StyleContext::add_provider_for_screen(&screen, &provider, 0),
        Err(e) => error!("{}", e),
    };
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
            let Rectangle { x, y, width, height } = geometry;
            let model = monitor.get_model().unwrap_or("".to_string());
            println!(
                "Monitor {}: {} @ {}x{} x: {} y: {}",
                i, model, width, height, x, y
            );
        }
    }
}
