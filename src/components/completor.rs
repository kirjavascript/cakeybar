use crate::bar::Bar;
use crate::config::ConfigGroup;
use crate::components::Component;
use crate::wm::ipc::parser::parse_message;
use crate::wm::events::Event;
use crate::wm;

use gtk::prelude::*;
use gdk::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;

pub struct Completor {
    wrapper: gtk::Box,
    window_opt: Rc<RefCell<Option<gtk::Window>>>,
}

impl Component for Completor {
    fn destroy(&self) {
        self.wrapper.destroy();
        if self.window_opt.borrow().is_some() {
            self.window_opt.borrow().as_ref().unwrap().destroy();
        }
    }
}

impl Completor {
    pub fn init(config: ConfigGroup, bar: &mut Bar, container: &gtk::Box) {

        // create wrapper

        let wrapper = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        super::init_widget(&wrapper, &config, bar, container);
        wrapper.show();

        let window_opt: Rc<RefCell<Option<gtk::Window>>>
            = Rc::new(RefCell::new(None));

        // get focus event

        let event_type = Event::Focus(config.name.clone());
        let wm_util = bar.wm_util.clone();
        let event = bar.wm_util.add_listener(event_type,
            clone!((window_opt, wrapper) move |_| {
                if window_opt.borrow().is_some() {
                    return
                }

                // get coords
                let rect = wrapper.get_allocation();
                let (_zo, xo, yo) = wrapper.get_window().unwrap().get_origin();
                let x = xo + rect.x;
                let y = yo + rect.y;

                // create window

                let window = gtk::Window::new(gtk::WindowType::Toplevel);
                wm::gtk::set_transparent(&window);
                window.set_type_hint(gdk::WindowTypeHint::PopupMenu);
                window.set_skip_pager_hint(false);
                window.set_skip_taskbar_hint(false);
                window.set_decorated(false);
                window.set_keep_above(true);
                window.set_title("");
                window.move_(x, y);
                window.resize(1, 1);
                window.set_resizable(false);
                window.show();
                window.stick();
                wm::gtk::disable_shadow(&window);

                let destroy = clone!((window_opt, window) move || {
                    window_opt.borrow_mut().take();
                    window.destroy();
                });

                // TODO: set active on wrapper

                wm_util.add_window(&window);

                // add entry

                let entry = gtk::Entry::new();
                window.add(&entry);
                entry.set_has_frame(false);
                entry.show();
                entry.grab_focus();

                // styles
                // TODO: use super::init_widget instead
                WidgetExt::set_name(&entry, &config.name);
                if let Some(ctx) = entry.get_style_context() {
                    ctx.add_class("active");
                }

                // events

                entry.connect_activate(clone!((wm_util, destroy) move |e| {
                    if let Some(text) = e.get_text() {
                        if text.starts_with(":") {
                            if let Ok(cmd) = parse_message(&text[1..]) {
                                wm_util.run_command(cmd);
                            }
                        } else {
                            crate::util::run_command(text.to_owned());
                        }
                        e.destroy();
                        destroy();
                    }
                }));

                window.connect_delete_event(clone!(window_opt move |e, _| {
                    window_opt.borrow_mut().take();
                    gtk::Inhibit(false)
                }));

                // TODO: connect escape, empty or close or <C-c>

                entry.connect_focus_out_event(clone!(window move |e, _| {
                    wm::gtk::keyboard_grab(&window);
                    e.grab_focus();
                    gtk::Inhibit(false)
                }));

                // swallow right click
                entry.connect_event(|_, e| {
                    Inhibit(e.get_button() == Some(3))
                });

                // stop window moving
                window.connect_configure_event(clone!(window_opt move |w, e| {
                    if window_opt.borrow().is_some()
                        && Some((x as f64, y as f64)) != e.get_coords() {
                        w.move_(x, y);
                    }
                    false
                }));

                *window_opt.borrow_mut() = Some(window);
            }
        ));

        bar.add_component(Box::new(Completor {
            wrapper,
            window_opt,
        }));
    }
}
