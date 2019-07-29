// use std::sync::mpsc;
use std::thread;
use std::collections::HashMap;

use crate::wm;
// use crate::wm::events::{Event, EventValue};

type XWindowData = (i16, i16, u16, u16, String);

const GEOMETRY_NOTIFY: u8 = 150;

pub fn listen(wm_util: &crate::wm::WMUtil) {
    // let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        match xcb::Connection::connect(None) {
            Ok((conn, screen_num)) => {

                let screen = conn.get_setup().roots().nth(screen_num as usize).unwrap();
                let atoms = wm::atom::Atoms::new(&conn);
                let mut windows = get_initial_windows(&conn, &atoms, &screen);
                println!("{:?}", windows);

                // override redirect on decorations?
                //
                // account for border width

                // iterate and == diff on the decoration side

                xcb::change_window_attributes(
                    &conn,
                    screen.root(),
                    &[
                        (
                            xcb::CW_EVENT_MASK,
                            xcb::EVENT_MASK_SUBSTRUCTURE_NOTIFY,
                        ),
                    ],
                );

                conn.flush();

                // map notify event

                loop {
                    match conn.wait_for_event() {
                        Some(event) => {
                            match event.response_type() {
                                xcb::CREATE_NOTIFY => {
                                    println!("{:#?}", "create");
                                },
                                xcb::DESTROY_NOTIFY => {
                                    println!("{:#?}", "destroy");
                                },
                                xcb::PROPERTY_NOTIFY => {
                                    let event: &xcb::PropertyNotifyEvent = unsafe {
                                        xcb::cast_event(&event)
                                    };

                                    if event.atom() == atoms.get(wm::atom::_NET_WM_NAME) {
                                        let xcb_window = event.window();
                                        let window = windows.get_mut(&xcb_window);
                                        if let Some(window) = window {
                                            let name = get_name(&conn, xcb_window);
                                            if window.4 != name {
                                                window.4 = name;
                                                println!("{:#?}", windows);
                                            }
                                        }
                                    }
                                    // println!("{:#?}", "prop");
                                },
                                xcb::CONFIGURE_NOTIFY => {
                                    // println!("{:#?}", "config");
                                },
                                GEOMETRY_NOTIFY => {
                                    let event: &xcb::ConfigureNotifyEvent = unsafe {
                                        xcb::cast_event(&event)
                                    };

                                    let window = event.window();
                                    let name = get_name(&conn, window);

                                    windows.insert(window, (
                                        event.x(),
                                        event.y(),
                                        event.width(),
                                        event.height(),
                                        name,
                                    ));
                                    println!("{:#?}", windows);
                                },
                                _ => {
                                    // println!("{:#?}", event.response_type());
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

fn get_name(conn: &xcb::Connection, window: xcb::Window) -> String {
    match xcb_util::icccm::get_wm_name(conn, window).get_reply() {
        Ok(reply) => reply.name().to_string(),
        Err(_) => "".to_string(),
    }
}

fn get_client_list(
    conn: &xcb::Connection,
    atoms: &wm::atom::Atoms,
    screen: &xcb::Screen,
) -> Vec<xcb::Window> {
    xcb::get_property(
        &conn,
        false,
        screen.root(),
        atoms.get(wm::atom::_NET_CLIENT_LIST),
        xcb::ATOM_WINDOW,
        0,
        8,
    )
        .get_reply()
        .and_then(|r| Ok(r.value().to_vec()))
        .unwrap_or_else(|_| Vec::new())
}

fn add_window(
    conn: &xcb::Connection,
    window: xcb::Window,
) -> XWindowData {
    xcb::change_window_attributes(
        &conn,
        window,
        &[
            (
                xcb::CW_EVENT_MASK,
                xcb::EVENT_MASK_PROPERTY_CHANGE | xcb::EVENT_MASK_STRUCTURE_NOTIFY
            ),
        ],
    );
    let name = get_name(&conn, window);
    let (x, y, width, height) = xcb::get_geometry(&conn, window)
        .get_reply()
        .map(|geom| (geom.x(), geom.y(), geom.width(), geom.height()))
        .unwrap_or_else(|_| (0, 0, 0, 0));

    (x, y, width, height, name)
}

fn get_initial_windows(
    conn: &xcb::Connection,
    atoms: &wm::atom::Atoms,
    screen: &xcb::Screen,
) -> HashMap<xcb::Window, XWindowData> {
    let mut windows = HashMap::new();
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
        Ok(reply) => {
            for window in reply.value() {
                xcb::change_window_attributes(
                    &conn,
                    *window,
                    &[
                        (
                            xcb::CW_EVENT_MASK,
                            xcb::EVENT_MASK_PROPERTY_CHANGE | xcb::EVENT_MASK_STRUCTURE_NOTIFY
                        ),
                    ],
                );
                let name = get_name(&conn, *window);
                let (x, y, width, height) = xcb::get_geometry(&conn, *window)
                    .get_reply()
                    .map(|geom| (geom.x(), geom.y(), geom.width(), geom.height()))
                    .unwrap_or_else(|_| (0, 0, 0, 0));

                windows.insert(*window, (x, y, width, height, name));
            }
        },
        _ => {},
    }
    windows
}
