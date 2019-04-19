use gdk::ScrollDirection;
use gtk::prelude::*;
use gtk::{Orientation, Overlay, Rectangle, Window, WindowType};
use glib::SignalHandlerId;
use glib::translate::{ToGlib, from_glib};

use std::cell::RefCell;
use std::rc::Rc;

use crate::{wm, NAME};
use crate::components::{load_component, Component};
use crate::config::ConfigGroup;
use crate::wm::ipc::commands::Selectors;

pub struct Bar {
    pub config: ConfigGroup,
    pub components: Vec<Box<dyn Component>>,
    pub overlay: Overlay,
    pub container: gtk::Box,
    pub wm_util: wm::WMUtil,
    event_ids: Vec<SignalHandlerId>,
    pub window: Window,
}

impl wm::Window for Bar {
    fn destroy(&self) {
        self.unload();
        self.window.destroy();
    }

    fn show(&self) {
        self.window.show();
    }

    fn hide(&self) {
        self.window.hide();
    }

    fn relayout(&self) {
        self.window.resize(1, 1);
    }

    fn to_window(&self) -> Window {
        self.unload();
        self.window.clone()
    }

    fn matches_selectors(&self, selectors: &Selectors) -> bool {
        selectors.contains_id(&self.config.name) || {
            let class = self.config.get_string("class");
            class.is_some() && selectors.contains_class(&class.unwrap())
        }
    }
}

impl Bar {
    pub fn new(
        config: ConfigGroup,
        wm_util: wm::WMUtil,
        monitor: &Rectangle,
        existing_window: Option<Window>,
    ) -> Bar {
        // TODO: check if the type differs to existing window
        let reserve_space = config.get_bool_or("reserve-space", true);
        let window_type = if reserve_space {
            WindowType::Toplevel
        } else {
            WindowType::Popup
        };

        // use existing or create new window
        let is_new = existing_window.is_none();
        let window = if let Some(existing) = existing_window {
            existing
        } else {
            let window = Window::new(window_type);
            wm_util.add_window(&window);
            window
        };

        // set base values
        window.set_default_size(monitor.width, 1);
        if is_new {
            window.set_title(NAME);
            #[allow(deprecated)]
            window.set_wmclass(NAME, NAME);
            window.set_type_hint(gdk::WindowTypeHint::Dock);
            window.set_keep_below(true);
            window.stick();

            wm::gtk::set_transparent(&window);
        }

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
        let size_id = window.connect_size_allocate(clone!((is_set, wm_util)
            move |window, rect| {
                let xpos = x;
                let ypos = if !is_top {
                    y + (height - rect.height)
                } else {
                    y
                };

                if !*is_set.borrow() || (xpos, ypos) != window.get_position() {
                    *is_set.borrow_mut() = true;
                    window.move_(xpos, ypos);
                    if reserve_space {
                        wm_util.set_padding(is_top, rect.height);
                    }
                    // set_strut crashes here :/
                }
            }
        ));

        // set .focused
        let focus_id = window.connect_enter_notify_event(clone!(container
            move |_, _| {
                if let Some(ctx) = container.get_style_context() {
                    StyleContextExt::add_class(&ctx, "focused");
                }
                Inhibit(false)
            })
        );
        let unfocus_id = window.connect_leave_notify_event(clone!(container
             move |_, _| {
                 if let Some(ctx) = container.get_style_context() {
                     StyleContextExt::remove_class(&ctx, "focused");
                 }
                 Inhibit(false)
             })
        );

        let event_ids = vec![size_id, focus_id, unfocus_id];

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
            event_ids,
        };

        bar.load_components();

        if is_new && bar.config.get_bool_or("disable-shadow", true) {
            wm::gtk::disable_shadow(&bar.window);
        }

        wm::gtk::set_strut(
            &bar.window,
            is_top,
            Rectangle {
                x: monitor.x,
                y: monitor.y,
                width: monitor.width,
                height: bar.window.get_size().1,
            },
        );

        bar
    }
        // builder pattern

    pub fn add_component(&mut self, component: Box<dyn Component>) {
        self.components.push(component);
    }

    pub fn load_components(&mut self) {
        let container = self.container.clone();
        for name in self.config.get_string_vec("layout") {
            let config_opt = self.wm_util.get_component_config(&name);
            if let Some(config) = config_opt {
                load_component(config, self, &container);
            } else {
                warn!("missing component #{}", name);
            }
        }
    }

    fn unload(&self) {
        // destroy components
        for component in self.components.iter() {
            component.destroy();
        }
        // remove events
        let window = self.window.clone();
        self.event_ids.iter().for_each(move |id| {
            // immutable signal copy
            window.disconnect(from_glib(id.to_glib()));
        });
        // remove widgets
        self.window.get_children().iter().for_each(|w| w.destroy());
    }

}
