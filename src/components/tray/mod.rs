use crate::components::{Component, ComponentParams};
use gdk::{WindowExt, RGBA};
use glib::translate::ToGlib;
use glib_sys::g_source_remove;
use gtk::prelude::*;
use gtk::Orientation;
use crate::util::Timer;

use crossbeam_channel::{self as channel, select};
use glib;
use std::sync::Arc;
use std::time::Duration;
use std::{process, thread};
use crate::wm;
use xcb;

mod manager;

// only used in the main thread
static mut TRAY_LOADED: bool = false;

#[derive(PartialEq)]
pub enum Action {
    Width(u16),
    Move(u32, u32),
    BgColor(u32),
    IconSize(u16),
    IconSpacing(u16),
    #[allow(dead_code)]
    Show,
    #[allow(dead_code)]
    Hide,
    Quit,
}

pub struct Tray {
    base_widget: gtk::Box,
    timer: Timer,
    sender: channel::Sender<Action>,
}

impl Component for Tray {
    fn destroy(&self) {
        self.base_widget.destroy();
        self.timer.remove();
        if let Err(err) = self.sender.send(Action::Quit) {
            error!("{}", err);
        }
        unsafe {
            TRAY_LOADED = false;
        }
    }
}

impl Tray {
    pub fn init(params: ComponentParams) {
        if unsafe { !TRAY_LOADED } {
            unsafe {
                TRAY_LOADED = true;
            }
            Tray::be_a_tray(params);
        } else {
            warn!("tray component is already loaded");
        }
    }
    pub fn be_a_tray(params: ComponentParams) {
        let ComponentParams { config, window, container, .. } = params;
        // extra surrounding base widget added for margins, etc
        let wrapper = gtk::Box::new(Orientation::Horizontal, 0);
        let base_widget = gtk::Box::new(Orientation::Horizontal, 0);
        base_widget.add(&wrapper);
        base_widget.show_all();
        super::init_widget(&base_widget, &config, &window, container);

        // communication
        let (s_main, r_main) = channel::unbounded();
        let (s_tray, r_tray) = channel::unbounded();

        // UI events/data

        // get bg color
        if let Some(ctx) = base_widget.get_style_context() {
            #[allow(deprecated)] // ctx.get_property doesn't work
            let RGBA {
                red, green, blue, ..
            } = ctx.get_background_color(gtk::StateFlags::NORMAL);
            let bg_color = (((red * 255.) as u32) << 16)
                + (((green * 255.) as u32) << 8)
                + (blue * 255.) as u32;
            s_main.send(Action::BgColor(bg_color)).unwrap();
        }

        // set icon size/spacing
        let icon_size = config.get_int_or("icon-size", 20);
        if icon_size != 20 {
            s_main.send(Action::IconSize(icon_size as u16)).unwrap();
        }
        let icon_spacing = config.get_int_or("icon-spacing", 0);
        if icon_spacing != 0 {
            s_main.send(Action::IconSpacing(icon_spacing as u16)).unwrap();
        }

        // send resize event
        wrapper.connect_size_allocate(clone!(s_main move |c, rect| {
            let w = c.get_window().unwrap();
            let (_zo, xo, yo) = w.get_origin();
            let y = (yo + (rect.y + ((rect.height - (icon_size as i32))/2))) as u32;
            let x = (xo + rect.x) as u32;
            if let Err(err) = s_main.send(Action::Move(x, y)) {
                error!("{}", err);
            }
        }));

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
                    return ();
                }

                manager.create();

                let (s_events, r_events) = channel::unbounded();
                thread::spawn(clone!(conn move || {
                    while let Some(event) = conn.wait_for_event() {
                        s_events.send(event).ok();
                    }
                }));

                let (r_signals, signal_destroy) = Tray::get_signals();

                loop {
                    select! {
                        // xcb events
                        recv(r_events) -> event_opt => {
                            if let Ok(event) = event_opt {
                                if let Some(code) = manager.handle_event(event) {
                                    info!("system tray exited with code {}", code);
                                    return;
                                }
                            } else {
                                error!("uhoh");
                            }
                        },
                        // gtk events
                        recv(r_main) -> action_opt => {
                            if let Ok(action) = action_opt {
                                if action == Action::Quit {
                                    manager.finish();
                                    signal_destroy();
                                    break;
                                } else {
                                    manager.handle_action(action);
                                }
                            }
                        },
                        // fullscreen tick
                        recv(fullscreen_tick) -> _ => {
                            if wm::xcb::check_fullscreen(&conn, &atoms, &screen) {
                                manager.hide();
                            } else {
                                manager.show();
                            }
                            conn.flush();
                        },
                        // signals
                        recv(r_signals) -> num => {
                            manager.finish();
                            process::exit(num.unwrap_or(0));
                        },
                    }
                }
            } else {
                error!("tray: could not connect to X server 😢");
            }
        });

        // receive events
        let timer = Timer::add_ms(10, clone!(base_widget move || {
            if let Ok(msg) = r_tray.try_recv() {
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
        }));

        window.add_component(Box::new(Tray {
            base_widget,
            timer,
            sender: s_main,
        }));
    }

    fn get_signals() -> (channel::Receiver<i32>, impl Fn()) {
        let (s, r) = channel::bounded(2);
        let id2 = glib::source::unix_signal_add(2, clone!(s move || {
            s.send(2).unwrap();
            gtk::Continue(false)
        })).to_glib(); // SIGINT
        let id15 = glib::source::unix_signal_add(15, move || {
            s.send(15).unwrap();
            gtk::Continue(false)
        }).to_glib(); // SIGTERM
        (r, move || {
            unsafe {
                g_source_remove(id2);
            }
            unsafe {
                g_source_remove(id15);
            }
        })
    }
}
