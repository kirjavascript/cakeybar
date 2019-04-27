use gtk::prelude::*;
use gtk::{Orientation, Overlay, Rectangle, Window, WindowType};
use glib::SignalHandlerId;
use glib::translate::{ToGlib, from_glib};

use std::cell::RefCell;
use std::rc::Rc;

use crate::components::{Component, ComponentParams, load_component};
use crate::config::ConfigGroup;
use crate::wm::ipc::commands::Selectors;
use crate::wm;

pub struct Float {
    config: ConfigGroup,
    components: Vec<Box<dyn Component>>,
    overlay: Overlay,
    container: gtk::Box,
    event_ids: Vec<SignalHandlerId>,
    window: Window,
}

impl Float {
    pub fn new(
        config: ConfigGroup,
        wm_util: &wm::WMUtil,
        monitor: &Rectangle,
        existing_window: Option<Window>,
    ) -> Float {
        // use existing or create new window
        let is_new = existing_window.is_none();
        let window = if let Some(existing) = existing_window {
            existing
        } else {
            let window = Window::new(WindowType::Toplevel);
            wm_util.add_gtk_window(&window);
            window
        };

        // TODO: grab close event
        // TODO: sticky

        // config: min-width, min-height, title

        // let x = config.get_int_or("x", 200);
        // let y = config.get_int_or("y", 200);

        // set base values
        if is_new {
            window.set_title(config.get_str_or("title", ""));
            window.set_type_hint(gdk::WindowTypeHint::PopupMenu);
            window.set_keep_below(true);
            window.set_skip_pager_hint(false);
            window.set_skip_taskbar_hint(false);
            window.set_decorated(false);
            window.stick();

            wm::gtk::set_transparent(&window);
        }

        // init container
        let container = gtk::Box::new(Orientation::Horizontal, 0);
        WidgetExt::set_name(&container, &config.name);
        WidgetExt::set_name(&window, &config.name);

        // get width/height from CSS context
        if let Some(ctx) = container.get_style_context() {
            let width = wm::gtk::get_style_property_uint(&ctx, "min-width");
            let height = wm::gtk::get_style_property_uint(&ctx, "min-height");
            window.resize(width.max(1) as i32, height.max(1) as i32);
        }

        // create overlay
        let overlay = Overlay::new();

        // connect everything up
        overlay.add(&container);
        window.add(&overlay);

        // get_pos
        // set_pos
        // monitor

        // set position
        let &Rectangle { x, y, .. } = monitor;
        let is_set = Rc::new(RefCell::new(false));
        // TODO: start at wrong side bug
        let size_id = window.connect_size_allocate(clone!(is_set
            move |window, _rect| {
                let xpos = x + 0;
                let ypos = y + 0;
                if !*is_set.borrow() || (xpos, ypos) != window.get_position() {
                    *is_set.borrow_mut() = true;
                    window.move_(xpos, ypos);
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
        if config.get_bool_or("hidden", false) {
            window.hide();
        }

        // create Float
        let float = Float {
            config,
            components: Vec::new(),
            overlay,
            container,
            window,
            event_ids,
        };

        if is_new && float.config.get_bool_or("disable-shadow", true) {
            wm::gtk::disable_shadow(&float.window);
        }

        if let Some(ctx) = float.container.get_style_context() {
            let left = wm::gtk::get_style_property_uint(&ctx, "left");
            info!("{}", left);
        }

        float
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

impl wm::Window for Float {
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
        if let Some(ctx) = self.container.get_style_context() {
            let width = wm::gtk::get_style_property_uint(&ctx, "min-width");
            let height = wm::gtk::get_style_property_uint(&ctx, "min-height");
            self.window.resize(width.max(1) as i32, height.max(1) as i32);
        }
    }

    fn to_window(&self) -> Window {
        self.unload();
        self.window.clone()
    }

    fn get_container(&self) -> &gtk::Box {
        &self.container
    }

    fn get_overlay(&self) -> &gtk::Overlay {
        &self.overlay
    }

    fn get_monitor_index(&self) -> usize {
        self.config.get_int_or("monitor", 0) as _
    }

    fn add_component(&mut self, component: Box<dyn Component>) {
        self.components.push(component);
    }

    fn load_component(&mut self, config: ConfigGroup, container: &gtk::Box, wm_util: &wm::WMUtil) {
        load_component(ComponentParams {
            container,
            config,
            window: Box::new(self),
            wm_util,
        });
    }

    fn matches_selectors(&self, selectors: &Selectors) -> bool {
        selectors.contains_id(&self.config.name) || {
            let class = self.config.get_string("class");
            class.is_some() && selectors.contains_class(&class.unwrap())
        }
    }
}
