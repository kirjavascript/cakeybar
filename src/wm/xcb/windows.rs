// use std::sync::mpsc;
use std::thread;


use crate::wm;
// use crate::wm::events::{Event, EventValue};

pub fn listen(wm_util: &crate::wm::WMUtil) {
    // let (tx, rx) = mpsc::channel();

    // let monitors = wm::gtk::get_monitor_coords();

    thread::spawn(move || {
        match xcb::Connection::connect(None) {
            Ok((conn, screen_num)) => {

                let screen = conn.get_setup().roots().nth(screen_num as usize).unwrap();
                let atoms = wm::atom::Atoms::new(&conn);
                // TODO: make a hashmap
                let windows = get_windows(&conn, &atoms, &screen);
                println!("{:#?}", windows);

                // check new window event (track root? MAP_NOTIFY)
                // check window delete event
                // override redirect on decorations?
                // say which one updated (newtype?)

                for window in windows {
                    xcb::change_window_attributes(
                        &conn,
                        window,
                        &[
                        // (xcb::CW_EVENT_MASK, xcb::EVENT_MASK_PROPERTY_CHANGE),
                            (xcb::CW_EVENT_MASK, xcb::EVENT_MASK_STRUCTURE_NOTIFY),
                        ],
                    );
                    // get window name
                }

                conn.flush();

                // map notify event

                loop {
                    match conn.wait_for_event() {
                        Some(event) => {
                            match event.response_type() {
                                xcb::CONFIGURE_NOTIFY => {},
                                _ => {
                                    let event: &xcb::ConfigureNotifyEvent = unsafe {
                                        xcb::cast_event(&event)
                                    };

                                    let window = event.window();

            let name = match xcb_util::icccm::get_wm_name(&conn, window).get_reply() {
                Ok(reply) => reply.name().to_string(),
                Err(_) => "".to_string(),
            };
            println!("{:#?}", name);

                                    println!("{:?} {:?} {:?} {:?} {:#?}",
                                         event.x(),
                                         event.y(),
                                         event.width(),
                                         event.height(),
                                         event.window(),
                                    );
                                },
                            }
                        }
                        None => {
                            // tx.send(Err(format!("xcb: no events (?)"))).unwrap();
                            break;
                        }
                    }
                }
            },
            Err(err) => {
                // tx.send(Err(err.to_string())).unwrap();
            },
        }
    });

    // gtk::timeout_add(10, clone!(wm_util move || {
    //     if let Ok(msg_result) = rx.try_recv() {
    //         match msg_result {
    //             Ok(msg) => {
    //                 match msg {
    //                     XCBMsg::WindowTitle(value) => {
    //                         wm_util.emit_value(
    //                             Event::Window,
    //                             EventValue::String(value),
    //                         );
    //                     },
    //                     XCBMsg::Workspace(workspaces) => {
    //                         wm_util.emit_value(
    //                             Event::Workspace,
    //                             EventValue::Workspaces(workspaces),
    //                         );
    //                     },
    //                 }
    //             },
    //             Err(err) => {
    //                 warn!("{}, restarting thread", err.to_lowercase());
    //                 gtk::timeout_add(1000, clone!(wm_util move || {
    //                     listen(&wm_util);
    //                     gtk::Continue(false)
    //                 }));
    //                 return gtk::Continue(false);
    //             },
    //         };
    //     }
    //     gtk::Continue(true)
    // }));
}


fn get_windows(
    conn: &xcb::Connection,
    atoms: &wm::atom::Atoms,
    screen: &xcb::Screen,
) -> Vec<xcb::Window> {
    let cookie = xcb::get_property(
        &conn,
        false,
        screen.root(),
        atoms.get(wm::atom::_NET_CLIENT_LIST),
        xcb::ATOM_WINDOW,
        0,
        8,
    );
    match cookie.get_reply() {
        Ok(reply) => reply.value().to_vec(),
        Err(_) => Vec::new(),
    }
}
