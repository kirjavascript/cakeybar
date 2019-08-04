use std::sync::mpsc;
use std::thread;
use std::collections::HashMap;

use crate::wm;
use crate::wm::events::{Event, EventValue};

#[derive(Debug, Clone)]
pub struct XWindowData {
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub name: String,
    pub visible: bool,
}

const GEOMETRY_NOTIFY: u8 = 150;

pub fn listen(wm_util: &crate::wm::WMUtil) {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        match xcb::Connection::connect(None) {
            Ok((conn, screen_num)) => {

                let screen = conn.get_setup().roots().nth(screen_num as usize).unwrap();
                let atoms = wm::atom::Atoms::new(&conn);

                let mut windows = HashMap::new();

                for window in get_client_list(&conn, &atoms, &screen) {
                    windows.insert(window, add_window(&conn, window));
                }
                // init windows
                tx.send(Ok(windows.clone())).unwrap();

                xcb::change_window_attributes(
                    &conn,
                    screen.root(),
                    &[(xcb::CW_EVENT_MASK, xcb::EVENT_MASK_SUBSTRUCTURE_NOTIFY)],
                );

                conn.flush();

                loop {
                    match conn.wait_for_event() {
                        Some(event) => {
                            match event.response_type() {
                                xcb::MAP_NOTIFY => {
                                    let clients = get_client_list(&conn, &atoms, &screen);
                                    let new_clients = clients.iter()
                                        .filter(|c| !windows.keys().any(|k| &k == c))
                                        .collect::<Vec<_>>();

                                    if !new_clients.is_empty() {
                                        for window in new_clients {
                                            windows.insert(*window, add_window(&conn, *window));
                                            // add window
                                        }
                                        tx.send(Ok(windows.clone())).unwrap();
                                    }
                                },
                                xcb::DESTROY_NOTIFY => {
                                    let clients = get_client_list(&conn, &atoms, &screen);
                                    let removed_clients = windows.keys()
                                        .filter(|c| !clients.iter().any(|k| &k == c))
                                        .map(|c| c.clone())
                                        .collect::<Vec<xcb::Window>>();

                                    if !removed_clients.is_empty() {
                                        for window in removed_clients {
                                            windows.remove(&window);
                                            // remove window
                                        }
                                        tx.send(Ok(windows.clone())).unwrap();
                                    }
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
                                            if window.name != name {
                                                window.name = name;
                                                // name window
                                                tx.send(Ok(windows.clone())).unwrap();
                                            }
                                        }
                                    }
                                },
                                xcb::VISIBILITY_NOTIFY => {
                                    for (window, windowdata) in windows.iter_mut() {
                                        windowdata.visible = get_visible(&conn, *window);
                                    }
                                    tx.send(Ok(windows.clone())).unwrap();
                                },
                                GEOMETRY_NOTIFY => {
                                    let event: &xcb::ConfigureNotifyEvent = unsafe {
                                        xcb::cast_event(&event)
                                    };
                                    let window = event.window();

                                    let (name, visible) = {
                                        // try and get original window
                                        let windowdata = windows.get(&window);
                                        if let Some(window) = windowdata {
                                            (window.name.clone(), window.visible)
                                        } else {
                                            // fallback to xcb
                                            (
                                                get_name(&conn, window),
                                                get_visible(&conn, window),
                                            )
                                        }
                                    };

                                    // update window position

                                    windows.insert(window, XWindowData {
                                        x: event.x(),
                                        y: event.y(),
                                        width: event.width(),
                                        height: event.height(),
                                        name,
                                        visible,
                                    });
                                    // geom window
                                    tx.send(Ok(windows.clone())).unwrap();
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
        while let Ok(windows_result) = rx.try_recv() {
            match windows_result {
                Ok(windows) => {
                    wm_util.emit_value(
                        Event::Windows,
                        EventValue::Windows(windows),
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

fn get_name(conn: &xcb::Connection, window: xcb::Window) -> String {
    match xcb_util::icccm::get_wm_name(conn, window).get_reply() {
        Ok(reply) => reply.name().to_string(),
        Err(_) => "".to_string(),
    }
}

fn get_visible(conn: &xcb::Connection, window: xcb::Window) -> bool {
    xcb::get_window_attributes(conn, window)
        .get_reply()
        .map(|attrs| attrs.map_state() & 2 == 2)
        .unwrap_or_else(|_| false)
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
                xcb::EVENT_MASK_PROPERTY_CHANGE
                | xcb::EVENT_MASK_STRUCTURE_NOTIFY
                | xcb::EVENT_MASK_VISIBILITY_CHANGE
            ),
        ],
    );
    conn.flush();

    let name = get_name(&conn, window);
    let visible = get_visible(&conn, window);
    let (x, y, width, height) = xcb::get_geometry(&conn, window)
        .get_reply()
        .map(|geom| (geom.x(), geom.y(), geom.width(), geom.height()))
        .unwrap_or_else(|_| (0, 0, 0, 0));

    XWindowData { x, y, width, height, name, visible }
}
