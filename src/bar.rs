use {gdk, gtk, wm, cairo};
use gtk::prelude::*;
use gtk::{
    Window,
    WindowType,
    Orientation,
    Rectangle,
};
use gdk::{ScrollDirection, ScreenExt};

use {NAME, components};
use config::{ComponentConfig};

use std::cell::RefCell;
use std::rc::Rc;

pub struct Bar<'a> {
    pub config: &'a ComponentConfig,
    pub components: &'a Vec<ComponentConfig>,
    pub application: &'a gtk::Application,
    pub wm_util: &'a wm::WMUtil,
}

impl<'a> Bar<'a> {
    pub fn new(
        application: &'a gtk::Application,
        config: &'a ComponentConfig,
        components: &'a Vec<ComponentConfig>,
        wm_util: &'a wm::WMUtil,
    ) -> Bar<'a> {

        let bar = Bar { config, application, components, wm_util };

        let monitors = wm::gtk::get_monitors();
        let monitor_index = bar.config.get_int_or("monitor", 0);
        let monitor_option = monitors.get(monitor_index as usize);

        match monitor_option {
            None => {
                warn!(
                    "no monitor at index {}",
                    monitor_index
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
        let &Bar { wm_util, .. } = self;

        // transparency
        set_visual(&window, &None);
        window.connect_screen_changed(set_visual);
        window.connect_draw(draw);
        window.set_app_paintable(true);

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
            viewport.connect_scroll_event(clone!(wm_util move |_vp, e| {
                let direction = e.get_direction();
                let is_next = direction == ScrollDirection::Down;
                wm_util.cycle_workspace(is_next, monitor_index);
                Inhibit(true)
            }));
        }
        viewport.set_shadow_type(gtk::ShadowType::None);
        viewport.add(&container);
        window.add(&viewport);

        // set position
        let is_top = self.config.get_str_or("position", "top") == "top";
        let &Rectangle { x, y, height, .. } = monitor;
        let is_set = Rc::new(RefCell::new(false));
        window.connect_size_allocate(clone!((is_set, wm_util)
            move |window, rect| {
                let xpos = x;
                let ypos = if !is_top { y + (height - rect.height) } else { y };
                if !*is_set.borrow() || (xpos, ypos) != window.get_position() {
                    *is_set.borrow_mut() = true;
                    window.move_(xpos, ypos);
                    wm_util.set_padding(is_top, rect.height);
                    // set_strut crashes here :/
                }
            }
        ));

        // set .focused
        window.connect_enter_notify_event(clone!(container move |_, _| {
            if let Some(ctx) = container.get_style_context() {
                StyleContextExt::add_class(&ctx, "focused");
            }
            Inhibit(false)
        }));
        window.connect_leave_notify_event(clone!(container move |_, _| {
            if let Some(ctx) = container.get_style_context() {
                StyleContextExt::remove_class(&ctx, "focused");
            }
            Inhibit(false)
        }));

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
fn set_visual(window: &Window, _screen: &Option<gdk::Screen>) {
    if let Some(screen) = window.get_screen() {
        if let Some(visual) = screen.get_rgba_visual() {
            window.set_visual(&visual); // crucial for transparency
        }
    }
}

fn draw(_window: &Window, ctx: &cairo::Context) -> Inhibit {
    // crucial for transparency
    ctx.set_source_rgba(0.0, 0.0, 0.0, 0.0);
    ctx.set_operator(cairo::enums::Operator::Screen);
    ctx.paint();
    Inhibit(false)
}
