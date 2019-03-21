use crate::bar::Bar;
use crate::config::ConfigGroup;
use crate::components::Component;
use crate::wm::ipc::parser::parse_message;
use crate::wm::events::{Event, EventId};
use crate::wm::{self, WMUtil};

use gtk::prelude::*;
use gdk::prelude::*;
use glib::translate::{ToGlib, from_glib};
use gdk::Rectangle;

use std::cell::RefCell;
use std::rc::Rc;

mod autosuggest;

use autosuggest::Suggestions;

pub struct CommandInput {
    config: ConfigGroup,
    wrapper: gtk::Box,
    window_opt: Rc<RefCell<Option<gtk::Window>>>,
    event_id: EventId,
    wm_util: WMUtil,
}

impl Component for CommandInput {
    fn destroy(&self) {
        let event_type = Event::Focus(self.config.name.clone());
        self.wm_util.remove_listener(event_type, self.event_id);
        if self.window_opt.borrow().is_some() {
            self.window_opt.borrow().as_ref().unwrap().destroy();
        }
        self.wrapper.destroy();
    }
}


fn get_abs_rect(wrapper: &gtk::Box) -> Rectangle {
    let rect = wrapper.get_allocation();
    let (_zo, xo, yo) = wrapper.get_window().unwrap().get_origin();
    let x = xo + rect.x;
    let y = yo + rect.y;
    Rectangle {
        x, y,
        width: rect.width,
        height: rect.height,
    }
}

impl CommandInput {
    pub fn init(config: ConfigGroup, bar: &mut Bar, container: &gtk::Box) {

        // TODO: error message in wrapper
        // TODO: monitor focus
        // TODO: poll for blur
        // TODO: config: history
        // TODO: fix in bspwm / others (appears behind bar / test i3 reload)
        // TODO: TAB for word, Right for all

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

                let suggestions = Suggestions::load();

                // get rekt
                let Rectangle { x, y, width, height } = get_abs_rect(&wrapper);

                // create window
                let window = gtk::Window::new(gtk::WindowType::Toplevel);
                wm_util.add_window(&window);
                wm::gtk::set_transparent(&window);
                window.set_type_hint(gdk::WindowTypeHint::PopupMenu);
                window.set_skip_pager_hint(false);
                window.set_skip_taskbar_hint(false);
                window.set_decorated(false);
                window.set_keep_above(true);
                window.stick();
                window.set_title("");
                window.move_(x, y);
                window.resize(width, height);
                window.set_resizable(false);
                window.show();
                wm::gtk::disable_shadow(&window);

                // add entry

                let entry = gtk::Entry::new();
                entry.set_has_frame(false);

                // wrappers

                let overlay = gtk::Overlay::new();
                overlay.add(&entry);
                window.add(&overlay);

                // autosuggest

                let suggest = gtk::Label::new(None);
                WidgetExt::set_halign(&suggest, gtk::Align::Start);
                overlay.add_overlay(&suggest);
                overlay.set_overlay_pass_through(&suggest, true);

                // styles
                WidgetExt::set_name(&window, &config.name);
                if let Some(ctx) = entry.get_style_context() {
                    ctx.add_class("entry");
                }
                if let Some(ctx) = suggest.get_style_context() {
                    ctx.add_class("suggestion");
                }

                // events

                // snap to location stuff
                let size_id = wrapper.connect_size_allocate(
                    clone!(window move |wrapper, _| {
                        let rect = get_abs_rect(&wrapper);
                        window.move_(rect.x, rect.y);
                        window.resize(rect.width, rect.height);
                    })
                ).to_glib();

                // stop window moving
                window.connect_configure_event(clone!(wrapper move |w, e| {
                    let Rectangle { x, y, .. } = get_abs_rect(&wrapper);
                    if Some((x as f64, y as f64)) != e.get_coords() {
                        w.move_(x, y);
                    }
                    false
                }));

                // stop fullscreen
                window.connect_window_state_event(move |w, e| {
                    let state = e.get_new_window_state();
                    let is_fullscreen = !(state & gdk::WindowState::FULLSCREEN).is_empty();
                    if is_fullscreen {
                        w.unfullscreen();
                    }
                    Inhibit(false)
                });

                // focus out event
                let focus_id = {
                    entry.connect_focus_out_event(clone!(window move |e, _| {
                        wm::gtk::keyboard_grab(&window);
                        e.grab_focus_without_selecting();
                        Inhibit(false)
                    })).to_glib()
                };

                let destroy = clone!((window_opt, window, wrapper, entry) move || {
                    entry.disconnect(from_glib(focus_id));
                    wrapper.disconnect(from_glib(size_id));
                    window_opt.borrow_mut().take();
                    window.destroy();
                });

                entry.connect_activate(clone!((wm_util, destroy, suggestions) move |e| {
                    if let Some(text) = e.get_text() {
                        suggestions.select(&text);
                        // TODO: move into wm_util
                        if text.starts_with(":") {
                            if let Ok(cmd) = parse_message(&text[1..]) {
                                wm_util.run_command(cmd);
                            }
                        } else {
                            crate::util::run_command(text.to_owned());
                        }
                        destroy();
                    }
                }));

                window.connect_delete_event(clone!(destroy move |_, _| {
                    destroy();
                    gtk::Inhibit(false)
                }));

                // swallow right click
                entry.connect_event(|_, e| {
                    Inhibit(e.get_button() == Some(3))
                });

                // find suggestions
                entry.connect_property_text_notify(
                    clone!((suggest, suggestions) move |entry| {
                        let text = entry.get_buffer().get_text();

                        let len = entry.get_text_length() as usize;
                        match &suggestions.find(&text) {
                            Some(suggestion) if len != 0 => {
                                suggest.set_text(&format!(
                                    "{}{}",
                                    " ".repeat(len),
                                    &suggestion[len..],
                                ));
                            },
                            _ => {
                                suggest.set_text("");
                            },
                        }
                    })
                );

                // grab keycodes for destroying / completing
                entry.connect_key_press_event(
                    clone!((suggestions, destroy) move |entry, e| {
                        use gdk::enums::key::{Tab, Escape, Right};
                        let (code, mask) = (e.get_keyval(), e.get_state());
                        let is_escape = code == Escape;
                        let is_ctrlc = code == 99 && mask == gdk::ModifierType::CONTROL_MASK;

                        if is_escape || is_ctrlc {
                            destroy();
                        } else if code == Tab {
                            let text = entry.get_buffer().get_text();
                            if let Some(suggestion) = suggestions.find_word(&text) {
                                entry.set_text(&suggestion);
                                entry.set_position(-1);
                            }
                        } else if code == Right {
                            let text = entry.get_buffer().get_text();
                            if let Some(suggestion) = suggestions.find(&text) {
                                entry.set_text(&suggestion);
                                entry.set_position(-1);
                            }
                        }

                        Inhibit(false)
                    })
                );

                // show everything, grab keyboard

                window.show_all();
                wm::gtk::keyboard_grab(&window);
                entry.grab_focus();

                *window_opt.borrow_mut() = Some(window);
            }
        ));

        bar.add_component(Box::new(CommandInput {
            config,
            wrapper,
            window_opt,
            event_id: event,
            wm_util,
        }));
    }
}
