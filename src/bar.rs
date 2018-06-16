use {gdk, gtk};
use gtk::prelude::*;
use gtk::{
    Window,
    WindowType,
    Orientation,
    Box,
};
use gdk::ScrollDirection;

use {util, NAME, components};
use config::{BarConfig, Position, ComponentConfig};
use components::i3workspace::scroll_workspace;

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
        window.set_title(NAME);
        window.set_default_size(0, 1);
        window.set_type_hint(gdk::WindowTypeHint::Dock);
        window.set_wmclass(NAME, NAME);

        // attach container
        let container = Box::new(Orientation::Horizontal, 0);
        WidgetExt::set_name(&container, &self.config.name);
        WidgetExt::set_name(&window, &self.config.name);

        // attach scrollevent
        let monitor_index = self.config.monitor_index as i32;
        let viewport = gtk::Viewport::new(None, None);
        // set gdk::EventMask::SCROLL_MASK and disable 'smooth' scrolling
        viewport.add_events(2097152);
        // when scrolling, change workspace
        viewport.connect_scroll_event(move |_vp, e| {
            let direction = e.get_direction();
            let is_next = direction == ScrollDirection::Down;
            // change workspace (i3)
            scroll_workspace(is_next, monitor_index);
            Inhibit(true)
        });
        viewport.set_shadow_type(gtk::ShadowType::None);
        viewport.add(&container);
        window.add(&viewport);

        // set position
        let x = monitor.x;
        let y = match self.config.position {
            Position::Bottom => monitor.y + (monitor.height / 2),
            Position::Top => monitor.y,
        };
        window.move_(x, y);

        // show bar
        window.show_all();

        // load components
        components::load_components(&container, &self);

        // TODO: windowEnter set ::focus
        // window.connect_enter_notify_event(move |_, _evt| {
        //     Inhibit(true)
        // });
    }
}
