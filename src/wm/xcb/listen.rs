use std::sync::mpsc;
use std::thread;

use xcb_util::ewmh;

use crate::wm;
use crate::wm::workspace::Workspace;
use crate::wm::events::{Event, EventValue};

enum XCBMsg {
    WindowTitle(String),
    Workspace(Vec<Workspace>),
}

pub fn listen(wm_util: &crate::wm::WMUtil) {
    let (tx, rx) = mpsc::channel();

    let is_unknown = wm_util.get_wm_type() == crate::wm::WMType::Unknown;

    thread::spawn(move || {
        match wm::xcb::connect_ewmh() {
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

                                        tx.send(Ok(XCBMsg::WindowTitle(title))).unwrap();
                                    }

                                    // get workspaces

                                    // TODO: urgent / visible
                                    // WM_HINTS

                                    let is_workspace = is_unknown && (
                                        event_atom == conn.NUMBER_OF_DESKTOPS()
                                        || event_atom == conn.CURRENT_DESKTOP()
                                        || event_atom == conn.DESKTOP_NAMES()
                                    );

                                    if is_workspace {
                                        let monitors = wm::gtk::get_monitor_coords();
                                        let workspaces = wm::xcb::get_workspaces(&conn, screen_num, &monitors);

                                        tx.send(Ok(XCBMsg::Workspace(workspaces))).unwrap();

                                    }
                                },
                                _ => { },
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
        while let Ok(msg_result) = rx.try_recv() {
            match msg_result {
                Ok(msg) => {
                    match msg {
                        XCBMsg::WindowTitle(value) => {
                            wm_util.emit_value(
                                Event::WindowTitle,
                                EventValue::String(value),
                            );
                        },
                        XCBMsg::Workspace(workspaces) => {
                            wm_util.emit_value(
                                Event::Workspace,
                                EventValue::Workspaces(workspaces),
                            );
                        },
                    }
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
