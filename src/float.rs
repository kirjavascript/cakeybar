use gtk::prelude::*;
use gtk::{Orientation, Overlay, Rectangle, Window, WindowType};
use glib::SignalHandlerId;
use glib::translate::{ToGlib, from_glib};

use std::cell::RefCell;
use std::rc::Rc;

use crate::components::{Component, ComponentParams, load_component};
use crate::config::{ConfigGroup, Property};
use crate::wm::ipc::commands::Selectors;
use crate::wm;

pub struct Float {
    config: ConfigGroup,
    components: Vec<Box<dyn Component>>,
    overlay: Overlay,
    container: gtk::Box,
    event_ids: Vec<SignalHandlerId>,
    window: Rc<RefCell<RCWindow>>,
}

struct RCWindow {
    monitor: Rectangle,
    gtkwindow: Window,
    position: [Option<i32>; 4], // top, bottom, left, right
}

impl RCWindow {
    fn new(
        monitor: Rectangle,
        gtkwindow: Window,
        position: [Option<i32>; 4],
    ) -> RCWindow {
        RCWindow {
            monitor,
            gtkwindow,
            position,
        }
    }

    fn move_test(&self) {
        let window_rect = self.gtkwindow.get_allocation();
        let monitor_rect = &self.monitor;
        let [top, bottom, left, right] = self.position;
        let x = Self::calc_pos(left, right, monitor_rect.width, window_rect.width);
        let y = Self::calc_pos(top, bottom, monitor_rect.height, window_rect.height);
        self.set_pos(x, y);
    }

    fn set_pos(&self, x: i32, y: i32) {
        self.gtkwindow.move_(self.monitor.x + x, self.monitor.y + y);
    }

    fn calc_pos(one: Option<i32>, two: Option<i32>, msize: i32, wsize: i32) -> i32 {
        if one.is_some() && two.is_some() {
            let (one, two) = (one.unwrap(), two.unwrap());
            let centre = msize as f32 / 2. - wsize as f32 / 2.;
            centre as i32 + (two - one)
        } else if two.is_some() {
            msize - wsize - two.unwrap()
        } else if one.is_some() {
            one.unwrap()
        } else {
            0
        }
    }
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
        // TODO: move into set_pos
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

        // show window (needs to do this at least once)
        window.show_all();
        if config.get_bool_or("hidden", false) {
            window.hide();
        }

        if is_new && config.get_bool_or("disable-shadow", true) {
            wm::gtk::disable_shadow(&window);
        }

        let position = [
            config.get_int("top").map(|x| x as i32),
            config.get_int("bottom").map(|x| x as i32),
            config.get_int("left").map(|x| x as i32),
            config.get_int("right").map(|x| x as i32),
        ];

        let window = Rc::new(RefCell::new(
            RCWindow::new(*monitor, window, position)
        ));

        // TODO: move event ids into rcwindow

        let mut event_ids = vec![];

        // set .focused
        let focus_id = window.borrow().gtkwindow
            .connect_enter_notify_event(clone!(container
                move |_, _| {
                    if let Some(ctx) = container.get_style_context() {
                        StyleContextExt::add_class(&ctx, "focused");
                    }
                    Inhibit(false)
                })
            );
        event_ids.push(focus_id);

        let unfocus_id = window.borrow().gtkwindow
            .connect_leave_notify_event(clone!(container
                 move |_, _| {
                     if let Some(ctx) = container.get_style_context() {
                         StyleContextExt::remove_class(&ctx, "focused");
                     }
                     Inhibit(false)
                 })
            );
        event_ids.push(unfocus_id);

        // let &Rectangle { x, y, .. } = monitor;
        // let position = Rc::new(RefCell::new((0, 0)));
        // TODO: start at wrong side bug
        let size_id = window.borrow().gtkwindow
            .connect_size_allocate(clone!(window
                move |_window, _rect| {
                    window.borrow().move_test();
                    // let (x, y) = *position.borrow();
                    // window.move_(x, y);
                    // println!("{:#?}", (x, y));
                    // weak.borrow().set_pos();
                    // let xpos = x + 0;
                    // let ypos = y + 0;
                    // if !*is_set.borrow() || (xpos, ypos) != window.get_position() {
                    //     *is_set.borrow_mut() = true;
                    //     window.move_(xpos, ypos);
                    // }
                    // info!("{:?}", weak.upgrade().is_some());
                }
            ));
        event_ids.push(size_id);

        window.borrow().move_test();

        Float {
            config,
            components: Vec::new(),
            overlay,
            container,
            window,
            event_ids,
            // position,
        }
    }

    // fn set_pos(&self) {
    //     // do width / height
    //     let (top, left, right, bottom) = (
    //         self.config.get_int("top"),
    //         self.config.get_int("left"),
    //         self.config.get_int("right"),
    //         self.config.get_int("bottom"),
    //     );

    //     fn calc_pos(one: Option<i64>, two: Option<i64>, msize: i32, wsize: i32) -> i32 {
    //         if one.is_some() && two.is_some() {
    //             let (one, two) = (one.unwrap() as i32, two.unwrap() as i32);
    //             let centre = msize as f32 / 2. - wsize as f32 / 2.;
    //             centre as i32 + (two - one)
    //         } else if two.is_some() {
    //             msize - wsize - two.unwrap() as i32
    //         } else if one.is_some() {
    //             one.unwrap() as i32
    //         } else {
    //             0
    //         }
    //     }
    //     let window_rect = self.window.get_allocation();
    //     let monitor_rect = &self.monitor;
    //     println!("{:#?}", window_rect);

    //     let x = calc_pos(left, right, monitor_rect.width, window_rect.width);
    //     let y = calc_pos(top, bottom, monitor_rect.height, window_rect.height);

    //     // self.window.move_(x + monitor_rect.x, y + monitor_rect.y);

    //     // *self.position.borrow_mut() = (x, y);

    //     // move_resize
    //     // move_to_rect
    //     // get_frame_extents
    //     // get_allocation
    // }

    // fn get_pos(&self) -> (x, y) {
    //     let &Rectangle { x, y, .. } = &self.monitor;
    //     let (xpos, ypox) = self.window.get_position();
    // }

    //             let xpos = x + 0;
    //             let ypos = y + 0;
    //             if !*is_set.borrow() || (xpos, ypos) != window.get_position() {
    //                 *is_set.borrow_mut() = true;
    //                 window.move_(xpos, ypos);
    //             }
    fn unload(&self) {
        // destroy components
        for component in self.components.iter() {
            component.destroy();
        }
        // TODO: move into RCWindow
        // remove events
        let window = self.window.clone();
        self.event_ids.iter().for_each(move |id| {
            // immutable signal copy
            window.borrow().gtkwindow.disconnect(from_glib(id.to_glib()));
        });
        // remove widgets
        self.window.borrow().gtkwindow.get_children().iter().for_each(|w| w.destroy());
    }

}

impl wm::Window for Float {
    fn destroy(&self) {
        self.unload();
        self.window.borrow().gtkwindow.destroy();
    }

    fn show(&self) {
        self.window.borrow().gtkwindow.show();
    }

    fn hide(&self) {
        self.window.borrow().gtkwindow.hide();
    }

    fn relayout(&self) {
        if let Some(ctx) = self.container.get_style_context() {
            let width = wm::gtk::get_style_property_uint(&ctx, "min-width");
            let height = wm::gtk::get_style_property_uint(&ctx, "min-height");
            self.window.borrow().gtkwindow
                .resize(width.max(1) as i32, height.max(1) as i32);
        }
    }

    fn to_window(&self) -> Window {
        self.unload();
        self.window.borrow().gtkwindow.clone()
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
