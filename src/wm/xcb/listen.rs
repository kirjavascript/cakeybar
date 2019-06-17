use std::sync::mpsc;
use std::thread;

use xcb_util::ewmh;
use gdk::{DisplayExt, MonitorExt};

use crate::wm::events::{Event, EventValue};

pub fn connect() -> Result<(ewmh::Connection, i32), &'static str> {
    let (connection, screen_num) = xcb::Connection::connect(None)
        .map_err(|_| "could not connect to X server")?;
    let connection = ewmh::Connection::connect(connection)
        .map_err(|_| "cannot get EWMH connection")?;
    Ok((connection, screen_num))
}

fn get_monitor_coords() -> Vec<(i32, i32, String)> {
    let display = gdk::Display::get_default().unwrap();
    let mut monitors = Vec::new();
    for i in 0..display.get_n_monitors() {
        if let Some(monitor) = display.get_monitor(i) {
            let gtk::Rectangle { x, y, .. } = monitor.get_geometry();
            let name = monitor.get_model().unwrap_or_else(|| "[unknown]".to_string());
            monitors.push((x, y, name));
        }
    }
    monitors
}

pub fn listen(wm_util: &crate::wm::WMUtil) {
    let (tx, rx) = mpsc::channel();

    // get monitor coordinates

    let monitors = get_monitor_coords();

    thread::spawn(move || {
        match connect() {
            Ok((conn, screen_num)) => {

                let screen = conn.get_setup().roots().nth(screen_num as usize).unwrap();

                xcb::change_window_attributes(
                    &conn,
                    screen.root(),
                    &[(xcb::CW_EVENT_MASK, xcb::EVENT_MASK_PROPERTY_CHANGE)],
                );

                conn.flush();

                let mut current_window = xcb::NONE;

                loop {
                    match conn.wait_for_event() {
                        Some(event) => {
                            match event.response_type() {
                                xcb::PROPERTY_NOTIFY => {
                                    let event: &xcb::PropertyNotifyEvent = unsafe {
                                        xcb::cast_event(&event)
                                    };

                                    // get active window title
                                    let event_atom = event.atom();
                                    let is_active_window = event_atom == conn.ACTIVE_WINDOW();
                                    let is_title = is_active_window || event_atom == conn.WM_NAME();
                                    if is_title {
                                        let title = ewmh::get_active_window(&conn, screen_num)
                                            .get_reply()
                                            .and_then(|active_window| {
                                                if is_active_window {
                                                    if current_window != active_window {
                                                        // unsubscribe old window
                                                        if current_window != xcb::NONE {
                                                            xcb::change_window_attributes(
                                                                &conn,
                                                                current_window,
                                                                &[(xcb::CW_EVENT_MASK, xcb::EVENT_MASK_NO_EVENT)],
                                                            );
                                                        }
                                                        // subscribe to new one
                                                        if active_window != xcb::NONE {
                                                            xcb::change_window_attributes(
                                                                &conn,
                                                                active_window,
                                                                &[(xcb::CW_EVENT_MASK, xcb::EVENT_MASK_PROPERTY_CHANGE)],
                                                            );
                                                        }
                                                        current_window = active_window;
                                                        conn.flush();
                                                    }
                                                }
                                                ewmh::get_wm_name(&conn, active_window).get_reply()
                                            })
                                        .map(|reply| reply.string().to_owned())
                                            .unwrap_or_else(|_| "".to_owned());

                                        tx.send(Ok(title)).unwrap();
                                    }

                                    // get workspaces

                                    let is_workspace = event_atom == conn.NUMBER_OF_DESKTOPS()
                                        || event_atom == conn.CURRENT_DESKTOP()
                                        || event_atom == conn.DESKTOP_NAMES();

                                    if is_workspace {

                                        let number = ewmh::get_number_of_desktops(&conn, screen_num)
                                            .get_reply()
                                            .unwrap_or(0) as usize;

                                        let current = ewmh::get_current_desktop(&conn, screen_num)
                                            .get_reply()
                                            .unwrap_or(0) as usize;
                                        let names_reply = ewmh::get_desktop_names(&conn, screen_num).get_reply();
                                        let mut names = match names_reply {
                                            Ok(ref r) => r.strings(),
                                            Err(_) => Vec::new(),
                                        };

                                        let vp_reply = ewmh::get_desktop_viewport(&conn, screen_num).get_reply();

                                        let mut vp = match vp_reply {
                                            Ok(ref r) => r.desktop_viewports().iter().map(|vp| (vp.x, vp.y)).collect(),
                                            Err(_) => Vec::new(),
                                        };


                                        // println!("{:#?}", (names, vp, &monitors));

                                    }

                                    // NUMBER_OF_DESKTOPS,
                                    // CURRENT_DESKTOP,
                                    // DESKTOP_NAMES
                                    //
                                    // WM_HINTS
                                },
                                _ => {},
                            }
                        }
                        None => {
                            tx.send(Err(format!("xcb: no events (?)"))).unwrap();
                            break;
                        }
                    }
                }
            },
            Err(err) => {
                tx.send(Err(err.to_string())).unwrap();
            },
        }
    });

    gtk::timeout_add(10, clone!(wm_util move || {
        if let Ok(msg_result) = rx.try_recv() {
            match msg_result {
                Ok(msg) => {
                    // only window title currently received
                    wm_util.emit_value(
                        Event::Window,
                        EventValue::String(msg),
                        );
                },
                Err(err) => {
                    warn!("{}, restarting thread", err.to_lowercase());
                    gtk::timeout_add(1000, clone!(wm_util move || {
                        listen(&wm_util);
                        gtk::Continue(false)
                    }));
                    return gtk::Continue(false);
                },
            };
        }
        gtk::Continue(true)
    }));
}
