use crate::bar::Bar;
use crate::config::ConfigGroup;
use crate::components::Component;
use crate::wm::ipc::parser::parse_message;
use crate::wm::events::{Event, EventId};
use crate::wm::{self, WMUtil};

use gtk::prelude::*;
use gdk::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;

pub struct Completor {
    config: ConfigGroup,
    wrapper: gtk::Box,
    window_opt: Rc<RefCell<Option<gtk::Window>>>,
    event_id: EventId,
    wm_util: WMUtil,
}

impl Component for Completor {
    fn destroy(&self) {
        let event_type = Event::Focus(self.config.name.clone());
        self.wm_util.remove_listener(event_type, self.event_id);
        if self.window_opt.borrow().is_some() {
            self.window_opt.borrow().as_ref().unwrap().destroy();
        }
        self.wrapper.destroy();
    }
}

impl Completor {
    pub fn init(config: ConfigGroup, bar: &mut Bar, container: &gtk::Box) {

        // TODO: rename to input and add config

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
            clone!((window_opt, wrapper, wm_util, config) move |_| {
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

                // TODO: prompt = ""
                // TODO: complete on TAB
                // TODO: set active on wrapper
                // TODO: error message in wrapper

                wm_util.add_window(&window);

                // add entry

                let entry = gtk::Entry::new();
                window.add(&entry);
                entry.set_has_frame(false);
                entry.show();
                entry.grab_focus();

                // add completion

                let store = gtk::ListStore::new(&[gtk::Type::String]);
                let completion = gtk::EntryCompletion::new();
                completion.set_model(&store);
                completion.set_popup_completion(true);
                completion.set_minimum_key_length(0);
                completion.set_text_column(0);
                completion.set_inline_completion(true);
                entry.set_completion(&completion);

                unsafe { String::from_utf8_unchecked(
                    std::process::Command::new("ls")
                        .arg("/usr/bin").output().unwrap().stdout
                ).split("\n") }.for_each(|s| {
                    store.set(&store.append(), &[0], &[&s.to_string()]);
                });

                // TODO: double fork for commands

                // TODO: steal dmenu format

                // ls /usr/bin
                entry.show_all();

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
                                // TODO: dont run as subcommand of process
                                // because it closes :(
                                wm_util.run_command(cmd);
                            }
                        } else {
                            crate::util::run_command(text.to_owned());
                        }
                        e.destroy();
                        destroy();
                    }
                }));

                window.connect_delete_event(clone!(window_opt move |_, _| {
                    window_opt.borrow_mut().take();
                    gtk::Inhibit(false)
                }));

                entry.connect_focus_out_event(clone!(window move |e, _| {
                    wm::gtk::keyboard_grab(&window);
                    e.grab_focus();
                    gtk::Inhibit(false)
                }));

                // swallow right click
                entry.connect_event(|_, e| {
                    Inhibit(e.get_button() == Some(3))
                });

                // grab keycodes for destroying
                entry.connect_key_press_event(clone!((destroy, completion) move |_, e| {
                    let (code, mask) = (e.get_keyval(), e.get_state());
                    let is_escape = code == gdk::enums::key::Escape;
                    let is_ctrlc = code == gdk::enums::key::C && mask == gdk::ModifierType::CONTROL_MASK;
                    if is_escape || is_ctrlc {
                        destroy();
                    } else if code == gdk::enums::key::Tab {
                        // println!("{:#?}", q);
                        // completion.insert_prefix();
                    }
                    Inhibit(false)
                }));

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
            config,
            wrapper,
            window_opt,
            event_id: event,
            wm_util,
        }));
    }
}
