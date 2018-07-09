use gtk;
use xcb;

use std::thread;
use std::sync::mpsc;

use wm;
use wm::events::{Event, EventValue};

// TODO: encoding, high CPU, init

pub fn listen(wm_util: &::wm::WMUtil) {

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {

        if let Ok((conn, screen_num)) = xcb::Connection::connect(None) {
            let atoms = wm::atom::Atoms::new(&conn);
            let screen_num = screen_num as usize;

            let setup = conn.get_setup();
            let screen = setup.roots().nth(screen_num).unwrap();

            xcb::change_window_attributes_checked(&conn, screen.root(), &[
                (xcb::CW_EVENT_MASK, xcb::EVENT_MASK_PROPERTY_CHANGE),
            ]);

            conn.flush();

            let _active_window = atoms.get(wm::atom::_NET_ACTIVE_WINDOW);
            let _current_desktop = atoms.get(wm::atom::_NET_CURRENT_DESKTOP);
            let _visible_name = atoms.get(wm::atom::_NET_WM_VISIBLE_NAME);
            let _wm_name = atoms.get(wm::atom::_NET_WM_NAME);

            loop {
                match conn.wait_for_event() {
                    Some(event) => {
                        let response_type = event.response_type();

                        match response_type {
                            xcb::PROPERTY_NOTIFY => {
                                let event: &xcb::PropertyNotifyEvent = unsafe {
                                    xcb::cast_event(&event)
                                };
                                let event_atom = event.atom();
                                let is_title = event_atom == _active_window
                                    || event_atom == _current_desktop
                                    || event_atom == _visible_name
                                    || event_atom == _wm_name;

                                if is_title {
                                    let cookie = xcb::get_property(
                                        &conn,
                                        false,
                                        screen.root(),
                                        _active_window,
                                        xcb::ATOM_WINDOW,
                                        0,
                                        8,
                                    );

                                    match cookie.get_reply() {
                                        Ok(reply) => {
                                            let value: &[u32] = reply.value();
                                            let window = value[0];
                                            if window == 0 {
                                                tx.send(Ok(
                                                    "".to_string()
                                                )).unwrap();
                                            } else {
                                                let title = wm::xcb::get_string(
                                                    &conn,
                                                    window,
                                                    xcb::ATOM_WM_NAME,
                                                );
                                                tx.send(Ok(
                                                    title.trim().to_string()
                                                )).unwrap();
                                            }
                                        },
                                        Err(err) => {
                                            warn!("xcb cookie error {:?}", err);
                                        },
                                    }
                                }
                            },
                            _ => { },
                        }
                    },
                    None => {
                        tx.send(Err(format!("xcb: no events (?)"))).unwrap();
                        break;
                    }
                }
            }

        }
        else {
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
                    gtk::timeout_add(100, clone!(wm_util move || {
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
