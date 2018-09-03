use {gdk, gtk, wm, cairo, NAME, components};
use gtk::prelude::*;
use gtk::{
    Window,
    WindowType,
    Orientation,
    Overlay,
    Rectangle,
};
use gdk::{ScrollDirection, ScreenExt};

use std::cell::RefCell;
use std::rc::Rc;

use config::{ConfigGroup, Config};
use components::{Component, load_component};

pub struct Bar {
    pub config: ConfigGroup,
    pub components: Vec<Box<Component>>,
    pub wm_util: wm::WMUtil,
    pub overlay: Overlay,
    pub container: gtk::Box,
    window: Window,
}

impl Bar {
    pub fn new(
        config: ConfigGroup,
        wm_util: wm::WMUtil,
        monitor: &Rectangle,
    ) -> Bar {
        let window_type = if config.get_bool_or("float", false) {
            WindowType::Popup
        } else {
            WindowType::Toplevel
        };
        let window = Window::new(window_type);
        wm_util.add_window(&window);

        // set base values
        window.set_title(NAME);
        window.set_default_size(monitor.width, 1);
        window.set_type_hint(gdk::WindowTypeHint::Dock);
        #[allow(deprecated)]
        window.set_wmclass(NAME, NAME);
        window.set_keep_above(true);
        window.stick();

        // transparency
        Self::set_visual(&window, &None);
        window.connect_screen_changed(Self::set_visual);
        window.connect_draw(Self::draw);
        window.set_app_paintable(true);

        // init container
        let container = gtk::Box::new(Orientation::Horizontal, 0);
        WidgetExt::set_name(&container, &config.name);
        WidgetExt::set_name(&window, &config.name);

        // create overlay
        let overlay = Overlay::new();

        // attach scrollevent
        let monitor_index = config.get_int_or("monitor", 0) as i32;
        let viewport = gtk::Viewport::new(None, None);
        // set gdk::EventMask::SCROLL_MASK and disable 'smooth' scrolling
        viewport.add_events(2097152);
        // when scrolling, change workspace
        if config.get_bool_or("workspace-scroll", false) {
            viewport.connect_scroll_event(clone!(wm_util move |_vp, e| {
                let direction = e.get_direction();
                let is_next = direction == ScrollDirection::Down;
                wm_util.cycle_workspace(is_next, monitor_index);
                Inhibit(true)
            }));
        }
        viewport.set_shadow_type(gtk::ShadowType::None);

        // connect everything up
        overlay.add(&container);
        viewport.add(&overlay);
        window.add(&viewport);

        // set position
        let is_top = config.get_str_or("position", "top") == "top";
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

        // show window
        window.show_all();

        // create Bar
        let mut bar = Bar {
            config,
            components: Vec::new(),
            wm_util,
            overlay,
            container,
            window,
        };

        bar.load_components();

        wm::gtk::disable_shadow(&bar.window);

        wm::gtk::set_strut(&bar.window, is_top, Rectangle {
            x: monitor.x,
            y: monitor.y,
            width: monitor.width,
            height: bar.window.get_size().1,
        });

        bar
    }

    pub fn load_components(&mut self) {
        for name in self.config.get_string_vec("layout") {
            let config_opt = self.wm_util.get_component_config(&name);
            if let Some(config) = config_opt {
                let component = load_component(config, &self, None);
                if let Some(mut component) = component {
                    self.components.push(component);
                }
            } else {
                warn!("missing component {:?}", name);
            }
        }
    }

    pub fn show(&self) {
        self.window.show();
    }

    pub fn hide(&self) {
        self.window.hide();
    }

    pub fn destroy(&self) {
        for component in self.components.iter() {
            component.destroy();
        }
        self.window.destroy();
    }

    fn set_visual(window: &Window, _screen: &Option<gdk::Screen>) {
        if let Some(screen) = window.get_screen() {
            if let Some(visual) = screen.get_rgba_visual() {
                window.set_visual(&visual);
            }
        }
    }

    fn draw(_window: &Window, ctx: &cairo::Context) -> Inhibit {
        ctx.set_source_rgba(0.0, 0.0, 0.0, 0.0);
        ctx.set_operator(cairo::enums::Operator::Screen);
        ctx.paint();
        Inhibit(false)
    }

}
