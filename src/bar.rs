use {gdk, gtk, wm};
use gtk::prelude::*;
use gtk::{
    Window,
    WindowType,
    Orientation,
    Box,
    Rectangle,
};
use gdk::ScrollDirection;

use {util, NAME, components};
use config::{ComponentConfig};
use components::i3workspace::scroll_workspace;

#[derive(Debug)]
pub struct Bar<'a, 'b, 'c> {
    pub config: &'b ComponentConfig,
    pub components: &'c Vec<ComponentConfig>,
    pub application: &'a gtk::Application,
}

static mut INDEX: u32 = 0;

fn get_index() -> u32 { unsafe { INDEX += 1; INDEX - 1 } }

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

        // set role (to target with xcb)
        let window_role = format!("confectionary_{}", get_index());
        window.set_role(&window_role);

        // set base values
        window.set_title(NAME);
        window.set_default_size(monitor.width, 1);
        window.set_type_hint(gdk::WindowTypeHint::Dock);
        window.set_wmclass(NAME, NAME);

        // attach container
        let container = Box::new(Orientation::Horizontal, 0);
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
        {
            let position = self.config.get_str_or("position", "top").to_string();
            let &Rectangle { x, y, height, .. } = monitor;
            // TODO: fix 0,0 bug non positioning bug
            window.connect_size_allocate(enclose!(window_role move |window, rect| {
                let xpos = x;
                let ypos = match position.as_str() {
                    "bottom" => y + (height - rect.height),
                    _ => y,
                };
                // if (xpos, ypos) != window.get_position() {
                    window.move_(xpos, ypos);
                    // println!("{:#?}", rect.height);
                    wm::util::set_strut(window_role.clone());
                // }
            }));
        }

        // show bar
        window.show_all();

        wm::util::set_strut(window_role.clone());

        // load components
        components::load_components(&container, &self);

        // TODO: windowEnter set ::focus
        // window.connect_enter_notify_event(move |_, _evt| {
        //     Inhibit(true)
        // });
    }

}
