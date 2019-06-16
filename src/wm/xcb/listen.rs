use std::sync::mpsc;
use std::thread;
use xcb_util::ewmh;

use crate::wm::events::{Event, EventValue};

pub fn listen(wm_util: &crate::wm::WMUtil) {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        if let Ok((xcb_conn, screen_num)) = xcb::Connection::connect(None) {
            let root = xcb_conn.get_setup()
                .roots().nth(screen_num as usize).unwrap().root();

            let conn = ewmh::Connection::connect(xcb_conn)
                .map_err(|(e, _)| e).expect("no ewmh connection");

            xcb::change_window_attributes(
                &conn,
                root,
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

                                let event_atom = event.atom();
                                let is_title = event_atom == conn.ACTIVE_WINDOW()
                                    || event_atom == conn.WM_NAME();

                                if is_title {
                                    let title = ewmh::get_active_window(&conn, screen_num)
                                        .get_reply()
                                        .and_then(|active_window| {
                                            if current_window != active_window {
                                                // unsubscribe old window
                                                if current_window != xcb::NONE {
                                                    xcb::change_window_attributes_checked(
                                                        &conn,
                                                        current_window,
                                                        &[(xcb::CW_EVENT_MASK, xcb::EVENT_MASK_NO_EVENT)],
                                                    );
                                                }
                                                // subscribe to new one
                                                if active_window != xcb::NONE {
                                                    xcb::change_window_attributes_checked(
                                                        &conn,
                                                        active_window,
                                                        &[(xcb::CW_EVENT_MASK, xcb::EVENT_MASK_PROPERTY_CHANGE)],
                                                    );
                                                }
                                                current_window = active_window;
                                                conn.flush();
                                            }
                                            ewmh::get_wm_name(&conn, active_window).get_reply()
                                        })
                                        .map(|reply| reply.string().to_owned())
                                        .unwrap_or_else(|_| "".to_owned());

                                    tx.send(Ok(title)).unwrap();
                                }
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

        } else {
            tx.send(Err(format!("could not connect to X server"))).unwrap();
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
