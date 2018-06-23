use {gdk, gtk, wm};
use gtk::prelude::*;
use gtk::{
    Window,
    WindowType,
    Orientation,
    Rectangle,
};
use gdk::{ScrollDirection};

use {util, NAME, components};
use config::{ComponentConfig};
use components::i3workspace::scroll_workspace;

use std::process::Command;

#[derive(Debug)]
pub struct Bar<'a, 'b, 'c> {
    pub config: &'b ComponentConfig,
    pub components: &'c Vec<ComponentConfig>,
    pub application: &'a gtk::Application,
}

impl<'a, 'b, 'c> Bar<'a, 'b, 'c> {
    pub fn new(
        application: &'a gtk::Application,
        config: &'b ComponentConfig,
        components: &'c Vec<ComponentConfig>,
    ) -> Bar<'a, 'b, 'c> {

        let bar = Bar { config, application, components };

        let monitors = util::get_monitors();
        let monitor_index = bar.config.get_int_or("monitor", 0);
        let monitor_option = monitors.get(monitor_index as usize);

        match monitor_option {
            None => {
                eprintln!(
                    "warning: no monitor at index {}",
                    monitor_index,
                );
            },
            Some(monitor) => {
                bar.init(monitor);
            },
        }

        bar
    }


    fn init(&self, monitor: &Rectangle) {

        let window = Window::new(WindowType::Toplevel);
        self.application.add_window(&window);

        // set base values
        window.set_title(NAME);
        window.set_default_size(monitor.width, 1);
        window.set_type_hint(gdk::WindowTypeHint::Dock);
        window.set_wmclass(NAME, NAME);
        window.set_keep_above(true);
        window.stick();

        // attach container
        let container = gtk::Box::new(Orientation::Horizontal, 0);
        WidgetExt::set_name(&container, &self.config.name);
        WidgetExt::set_name(&window, &self.config.name);

        // attach scrollevent
        let monitor_index = self.config.get_int_or("monitor", 0) as i32;
        let viewport = gtk::Viewport::new(None, None);
        // set gdk::EventMask::SCROLL_MASK and disable 'smooth' scrolling
        viewport.add_events(2097152);
        // when scrolling, change workspace
        if self.config.get_bool_or("scroll_workspace", true) {
            viewport.connect_scroll_event(move |_vp, e| {
                let direction = e.get_direction();
                let is_next = direction == ScrollDirection::Down;
                // change workspace (i3)
                scroll_workspace(is_next, monitor_index);
                Inhibit(true)
            });
        }
        viewport.set_shadow_type(gtk::ShadowType::None);
        viewport.add(&container);
        window.add(&viewport);

        // set position
        let is_top = self.config.get_str_or("position", "top") == "top";
        {
            let &Rectangle { x, y, height, .. } = monitor;
            // TODO: fix 0,0 non positioning bug
            window.connect_size_allocate(move |window, rect| {
                let xpos = x;
                let ypos = if !is_top { y + (height - rect.height) } else { y };
                // if (xpos, ypos) != window.get_position() {
                    window.move_(xpos, ypos);
                    // set bspwm padding
                    let position = if is_top { "top" } else { "bottom" };
                    // TODO: thread
                    Command::new("bspc")
                        .arg("config")
                        .arg(format!("{}_padding", position))
                        .arg(format!("{}", rect.height))
                        .output()
                        .ok();
                // }
            });
        }

        // TODO: windowEnter set ::focus
        // window.connect_enter_notify_event(move |_, _evt| {
        //     Inhibit(true)
        // });

        // show bar
        window.show_all();

        // load components
        components::load_components(&container, &self);

        wm::gtk::set_strut(&window, is_top, Rectangle {
            x: monitor.x,
            y: monitor.y,
            width: monitor.width,
            height: window.get_size().1,
        });

    }


}
