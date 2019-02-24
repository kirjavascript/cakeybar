use crate::bar::Bar;
use crate::components::Component;
use crate::config::ConfigGroup;
use gtk::prelude::*;
use crate::wm;
use gdk::prelude::*;
use crate::wm::ipc::parser::parse_message;
use crate::wm::events::Event;

pub struct Completor {
    wrapper: gtk::Box,
}

impl Component for Completor {
    fn destroy(&self) {
        self.wrapper.destroy();
    }
}

impl Completor {
    pub fn init(config: ConfigGroup, bar: &mut Bar, container: &gtk::Box) {

        // create wrapper

        let wrapper = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        super::init_widget(&wrapper, &config, bar, container);
        wrapper.show();

        // get focus event

        let event_type = Event::Focus(config.name.clone());
        let wm_util = bar.wm_util.clone();
        let event = bar.wm_util.add_listener(event_type, clone!(wrapper
            move |_| {
                // get coords
                let rect = wrapper.get_allocation();
                let (_zo, xo, yo) = wrapper.get_window().unwrap().get_origin();
                let x = xo + rect.x;
                let y = yo + rect.y;

                // create window

                let window = gtk::Window::new(gtk::WindowType::Toplevel);
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

                wm::gtk::set_transparent(&window);
                wm::gtk::disable_shadow(&window);


                if let Some(ctx) = window.get_style_context() {
                    // ctx.add_class("completor");
                    // TODO: add class / id from parent
                }

                wm_util.add_window(&window);

                // add entry

                let entry = gtk::Entry::new();
                window.add(&entry);
                entry.set_has_frame(false);
                entry.show();

                entry.grab_focus();

                // events (TODO)

                entry.connect_activate(clone!((window, wm_util) move |e| {
                    if let Some(text) = e.get_text() {
                        if let Ok(cmd) = parse_message(&text) {
                            wm_util._run_command(cmd);
                        }
                        window.destroy();
                    }
                }));

                // TODO: connect escape, empty or close

                entry.connect_focus_out_event(clone!(window move |e, t| {
                    window.destroy();
                    gtk::Inhibit(false)
                }));

                entry.connect_event(|_, e| {
                    // swallow right click
                    Inhibit(e.get_button() == Some(3))
                });

            }
        ));

        bar.add_component(Box::new(Completor {
            wrapper,
        }));
    }
}
