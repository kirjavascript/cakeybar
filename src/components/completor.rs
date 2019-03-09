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

impl Completor {
    pub fn init(config: ConfigGroup, bar: &mut Bar, container: &gtk::Box) {

        // stolen from dmenu https://git.suckless.org/dmenu/file/LICENSE.html
        let source = config.get_string("source").unwrap_or_else(|| r#"
            #!/bin/sh

            cachedir="${XDG_CACHE_HOME:-"$HOME/.cache"}"
            cache="$cachedir/cakeybar_run"

            [ ! -e "$cachedir" ] && mkdir -p "$cachedir"

            IFS=:
            if stest -dqr -n "$cache" $PATH; then
                stest -flx $PATH | sort -u | tee "$cache"
            else
                cat "$cache"
            fi
        "#.to_string());

        // TODO: rename to ??? and add config
        // TODO: prompt = ""
        // TODO: complete on TAB
        // TODO: set active on wrapper
        // TODO: error message in wrapper
        // TODO: up history
        // TODO: replace dropdown with grey text
        // TODO: !shell
        // TODO: recency

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

                // TODO: replace with source
                unsafe { String::from_utf8_unchecked(
                    std::process::Command::new("/bin/sh")
                        .arg("-c").arg(&source).output().unwrap().stdout
                ).split("\n") }.for_each(|s| {
                    store.set(&store.append(), &[0], &[&s.to_string()]);
                });

                entry.show();

                // styles
                // TODO: use super::init_widget instead
                WidgetExt::set_name(&entry, &config.name);
                if let Some(ctx) = entry.get_style_context() {
                    ctx.add_class("active");
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

                let destroy = clone!((window_opt, window, wrapper) move || {
                    wrapper.disconnect(from_glib(size_id));
                    window_opt.borrow_mut().take();
                    window.destroy();
                });

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

                window.connect_delete_event(clone!(destroy move |_, _| {
                    destroy();
                    gtk::Inhibit(false)
                }));

                entry.connect_focus_out_event(clone!(window move |e, _| {
                    wm::gtk::keyboard_grab(&window);
                    e.grab_focus_without_selecting();
                    gtk::Inhibit(false)
                }));

                // swallow right click
                entry.connect_event(|_, e| {
                    Inhibit(e.get_button() == Some(3))
                });

                // grab keycodes for destroying
                entry.connect_key_press_event(clone!((destroy, completion) move |_, e| {
                    use gdk::enums::key::{Tab, Escape};
                    let (code, mask) = (e.get_keyval(), e.get_state());
                    let is_escape = code == Escape;
                    let is_ctrlc = code == 99 && mask == gdk::ModifierType::CONTROL_MASK;

                    if is_escape || is_ctrlc {
                        destroy();
                    } else if code == Tab {
                        // println!("{:#?}", q);
                        // completion.insert_prefix();
                    }
                    Inhibit(false)
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
