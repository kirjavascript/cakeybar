use super::gdk::{Screen, ScreenExt, Rectangle};

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
