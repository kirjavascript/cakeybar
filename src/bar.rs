use super::{gdk, gtk};
use gio::prelude::*;
use gtk::prelude::*;
use gtk::{
    Window,
    WindowType,

    // Box,
    // Label,
    // WidgetExt,
    // Orientation,
    // Image,
    // CssProvider,
    // STYLE_PROVIDER_PRIORITY_APPLICATION,
};

use super::config::{BarConfig, Position};
use super::util;

pub struct Bar {
    config: BarConfig,
}

impl Bar {
    pub fn new(application: &gtk::Application, config: BarConfig) -> Bar {
        let monitors = util::get_monitors();
        let monitor_index = config.get_monitor_index();
        let monitor_option = monitors.get(monitor_index);

        let bar = Bar {
            config: config,
        };

        match monitor_option {
            Some(monitor) => {
                let mut window = Window::new(WindowType::Toplevel);
                application.add_window(&window);

                window.set_title(super::NAME);
                window.set_default_size(0, 27);
                window.set_type_hint(gdk::WindowTypeHint::Dock);

                // set position
                let x = monitor.x;
                let y = match bar.config.get_position() {
                    Position::bottom => monitor.y + (monitor.height / 2),
                    Position::top => monitor.y,
                };
                window.move_(x, y);

                window.show_all();
            },
            None => {
                eprintln!("warning: no monitor at index {}", monitor_index);
            },
        }

        bar
    }
}
