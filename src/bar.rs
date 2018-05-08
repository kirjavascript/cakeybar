use super::{gdk, gtk};
use gio::prelude::*;
use gtk::prelude::*;
use gtk::{
    Window,
    WindowType,
    // WidgetExt,

    // Box,
    Label,
    // Orientation,
    // Image,
};

use super::config::{BarConfig, Position};
use super::util;

pub struct Bar {
    config: BarConfig,
}

impl Bar {
    pub fn new(application: &gtk::Application, config: BarConfig) -> Bar {
        let monitors = util::get_monitors();
        let monitor_option = monitors.get(config.monitor_index);

        let bar = Bar {
            config: config,
        };

        match monitor_option {
            Some(monitor) => {
                // load bar
                let mut window = Window::new(WindowType::Toplevel);
                application.add_window(&window);

                window.set_title(super::NAME);
                window.set_default_size(0, 27);
                window.set_type_hint(gdk::WindowTypeHint::Dock);
                WidgetExt::set_name(&window, &bar.config.name);

    let label = Label::new(None);
    label.set_text(&"hello world");
    window.add(&label);

                // set position
                let x = monitor.x;
                let y = match bar.config.position {
                    Position::Bottom => monitor.y + (monitor.height / 2),
                    Position::Top => monitor.y,
                };
                window.move_(x, y);

                window.show_all();
            },
            None => {
                eprintln!("warning: no monitor at index {}", bar.config.monitor_index);
            },
        }

        bar
    }
}
