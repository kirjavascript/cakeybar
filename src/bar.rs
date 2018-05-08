use super::{gdk, gtk};
use gio::prelude::*;
use gtk::prelude::*;
use gtk::{
    Window,
    WindowType,
    // WidgetExt,

    Box,
    Label,
    Orientation,
    Image,
};

use std::path::Path;

use super::config::{BarConfig, Position};
use super::util;

pub struct Bar {
    config: BarConfig,
}

impl Bar {
    pub fn new(application: &gtk::Application, config: BarConfig) -> Bar {
        let monitors = util::get_monitors();
        let monitor_option = monitors.get(config.monitor_index);

        match monitor_option {
            Some(monitor) => {
                // load bar
                let mut window = Window::new(WindowType::Toplevel);
                application.add_window(&window);

                window.set_title(super::NAME);
                window.set_default_size(0, 27);
                window.set_type_hint(gdk::WindowTypeHint::Dock);
                let container = Box::new(Orientation::Horizontal, 10);
                WidgetExt::set_name(&window, &config.name);
                WidgetExt::set_name(&container, &config.name);

    let img: Image = Image::new_from_file(Path::new("./example/icon.svg"));
    container.add(&img);

    let label = Label::new(None);
    label.set_text(&"hello world");
    // label.set_margin_left(10);
    container.add(&label);

    window.add(&container);
    window.show_all();


                // set position
                let x = monitor.x;
                let y = match config.position {
                    Position::Bottom => monitor.y + (monitor.height / 2),
                    Position::Top => monitor.y,
                };
                window.move_(x, y);

                window.show_all();

            },
            None => {
                eprintln!("warning: no monitor at index {}", config.monitor_index);
            },
        }

        Bar {
            config: config,
        }
    }
}
