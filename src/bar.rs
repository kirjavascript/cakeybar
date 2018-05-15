use super::{gdk, gtk};
use gio::prelude::*;
use gtk::prelude::*;
use gtk::{
    Window,
    WindowType,
    Orientation,
    Box,
};

use super::{util, NAME};
use super::config::{BarConfig, Position, ComponentConfig, Property};
use super::components;

#[derive(Debug)]
pub struct Bar<'a, 'b, 'c> {
    pub config: &'b BarConfig,
    pub components: &'c Vec<ComponentConfig>,
    pub application: &'a gtk::Application,
}

impl<'a, 'b, 'c> Bar<'a, 'b, 'c> {
    pub fn new(
        application: &'a gtk::Application,
        config: &'b BarConfig,
        components: &'c Vec<ComponentConfig>,
    ) -> Bar<'a, 'b, 'c> {

        let bar = Bar { config, application, components };

        let monitors = util::get_monitors();
        let monitor_option = monitors.get(bar.config.monitor_index);

        match monitor_option {
            None => {
                eprintln!(
                    "warning: no monitor at index {}",
                    bar.config.monitor_index,
                );
            },
            Some(monitor) => {
                bar.init(monitor);
            },
        }

        bar
    }

    fn init(&self, monitor: &gtk::Rectangle) {

        let window = Window::new(WindowType::Toplevel);
        self.application.add_window(&window);

        // set base values
        window.set_title(super::NAME);
        window.set_default_size(0, 2);
        window.set_type_hint(gdk::WindowTypeHint::Dock);
        window.set_wmclass(NAME, NAME);

        // attach container
        let container = Box::new(Orientation::Horizontal, 10);
        // TODO: opacity popup
        WidgetExt::set_name(&window, &self.config.name);
        WidgetExt::set_name(&container, &self.config.name);
        window.add(&container);

        // set position
        let x = monitor.x;
        let y = match self.config.position {
            Position::Bottom => monitor.y + (monitor.height / 2),
            Position::Top => monitor.y,
        };
        window.move_(x, y);

        // load components
        components::load_components(&container, &self);

        // show bar
        window.show_all();

    }
}
