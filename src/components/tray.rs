use super::{Component, Bar, gtk, ComponentConfig};
use gtk::prelude::*;
use gtk::{Label};

pub struct Tray { }

// mutable statics should be safe within the same thread
static mut TRAY_LOADED: bool = false;

impl Component for Tray {
    fn init(container: &gtk::Box, config: &ComponentConfig, bar: &Bar) {
        if unsafe { !TRAY_LOADED } {
            unsafe { TRAY_LOADED = true; }
            Tray::load(container, config, bar);
        }
        else {
            eprintln!("Tray component is already loaded");
        }
    }
}
impl Tray{
    fn load(container: &gtk::Box, config: &ComponentConfig, _bar: &Bar) {
        let bg = config.get_str_or("background_color", "#000000");
        let icon_size = config.get_int_or("icon_size", 20);

        gtk::idle_add(move || {
            ::tray::as_subprocess();
            gtk::Continue(false)
        });
    }
}
