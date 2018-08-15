use super::{Component, Bar, gtk, ComponentConfig};
use gtk::prelude::*;
use gtk::{Orientation};
use gdk::{WindowExt, RGBA};

use xcb;
use glib;
use crossbeam_channel as channel;
use std::{thread, process};
use std::sync::Arc;
use std::time::Duration;
use wm;

mod manager;

pub struct Tray { }

// mutable statics should be safe within the same thread
static mut TRAY_LOADED: bool = false;

impl Component for Tray {
    fn init(container: &gtk::Box, config: &ComponentConfig, bar: &Bar) {
        if unsafe { !TRAY_LOADED } {
            unsafe { TRAY_LOADED = true; }
            Tray::be_a_tray(container, config, bar);
        }
        else {
            warn!("tray component is already loaded");
        }
    }
}

#[derive(Debug)]
pub enum Action {
    Width(u16),
    Move(u32, u32),
    BgColor(u32),
    IconSize(u16),
}

impl Tray {
    fn be_a_tray(container: &gtk::Box, config: &ComponentConfig, bar: &Bar) {
        // extra surrounding base widget added for margins, etc
        let wrapper = gtk::Box::new(Orientation::Horizontal, 0);
        let base_widget = gtk::Box::new(Orientation::Horizontal, 0);
        base_widget.add(&wrapper);
        base_widget.show_all();
        Self::init_widget(&base_widget, container, config, bar);

        // communication
        let (s_main, r_main) = channel::unbounded();
        let (s_tray, r_tray) = channel::unbounded();

        // UI events/data

        // get bg color
        if let Some(ctx) = base_widget.get_style_context() {
            #[allow(deprecated)] // ctx.get_property doesn't work
            let RGBA { red, green, blue, .. } = ctx.get_background_color(gtk::StateFlags::NORMAL);
            let bg_color = (((red * 255.) as u32) << 16) + (((green * 255.) as u32) << 8) + (blue * 255.) as u32;
            s_main.send(Action::BgColor(bg_color));
        }

        // set icon size
        let icon_size = config.get_int_or("icon-size", 20);
        if icon_size != 20 {
            s_main.send(Action::IconSize(icon_size as u16));
        }

        // send resize event
        wrapper.connect_size_allocate(move |c, rect| {
            let w = c.get_window().unwrap();
            let (_zo, xo, yo) = w.get_origin();
            let y = (yo + (rect.y + ((rect.height - (icon_size as i32))/2))) as u32;
            let x = (xo + rect.x) as u32;
            s_main.send(Action::Move(x, y));
        });

        let fullscreen_tick = channel::tick(Duration::from_millis(100));

        // start tray context
        thread::spawn(move || {

            if let Ok((conn, screen_num)) = xcb::Connection::connect(None) {
                let conn = Arc::new(conn);
                let atoms = wm::atom::Atoms::new(&conn);
                let screen_num = screen_num as usize;
                let setup = conn.get_setup();
                let screen = setup.roots().nth(screen_num).unwrap();

                let mut manager = manager::Manager::new(&conn, &atoms, &screen, s_tray);

                if !manager.is_selection_available() {
                    warn!("another system tray is already running");
                    return ()
                }

                manager.create();

                let (s_events, r_events) = channel::unbounded();
                thread::spawn(clone!(conn move || {
                    loop {
                        match conn.wait_for_event() {
                            Some(event) => { s_events.send(event); },
                            None => { break; }
                        }
                    }
                }));

                let r_signals = Tray::get_signals();

                loop {
                    select! {
                        // xcb events
                        recv(r_events, event_opt) => {
                            if let Some(event) = event_opt {
                                if let Some(code) = manager.handle_event(event) {
                                    info!("system tray exited with code {}", code);
                                    return ()
                                }
                            } else {
                                error!("tray: killed by XKillClient() maybe?");
                            }
                        },
                        // gtk events
                        recv(r_main, action_opt) => {
                            if let Some(action) = action_opt {
                                manager.handle_action(action);
                            }
                        },
                        // fullscreen tick
                        recv(fullscreen_tick) => {
                            if wm::xcb::check_fullscreen(&conn, &atoms, &screen) {
                                manager.hide();
                            } else {
                                manager.show();
                            }
                        },
                        // signals
                        recv(r_signals, num) => {
                            error!("dead");
                            manager.finish();
                            process::exit(num.unwrap_or(0));
                        },
                    }
                }

            }
            else {
                error!("tray: could not connect to X server 😢");
            }
        });

        // receive events
        gtk::timeout_add(10, move || {
            if let Some(msg) = r_tray.try_recv() {
                match msg {
                    Action::Width(w) => {
                        wrapper.set_size_request(w as i32, icon_size as i32);
                        // the next lines fix a background display bug
                        base_widget.hide();
                        base_widget.show();
                    },
                    _ => {},
                }
            }
            gtk::Continue(true)
        });

    }

    fn get_signals() -> channel::Receiver<i32> {
        let (s, r) = channel::bounded(2);
        glib::source::unix_signal_add(2, clone!(s move || {
            s.send(2);
            gtk::Continue(false)
        })); // SIGINT
        glib::source::unix_signal_add(15, move || {
            s.send(15);
            gtk::Continue(false)
        }); // SIGTERM
        r
    }
}
