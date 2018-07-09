use gtk;
use xcb;

use std::thread;
use std::sync::mpsc;

// use std::os::unix::net::{UnixStream};

use wm;
use wm::events::{Event, EventValue};

pub fn listen(wm_util: &::wm::WMUtil) {

    // let (tx, rx) = mpsc::channel();

    thread::spawn(move || {

        if let Ok((conn, screen_num)) = xcb::Connection::connect(None) {
            // let conn = Arc::new(conn);
            let atoms = wm::atom::Atoms::new(&conn);
            let screen_num = screen_num as usize;

            let setup = conn.get_setup();
            let screen = setup.roots().nth(screen_num).unwrap();

            xcb::change_window_attributes_checked(&conn, screen.root(), &[
                (xcb::CW_EVENT_MASK, xcb::EVENT_MASK_PROPERTY_CHANGE),
            ]);

            conn.flush();

            loop {
                match conn.wait_for_event() {
                    Some(event) => {
                        // tx.send(event);
                        let response_type = event.response_type();

                        match response_type {
                            xcb::PROPERTY_NOTIFY => {
                                let event: &xcb::PropertyNotifyEvent = unsafe {
                                    xcb::cast_event(&event)
                                };
                                println!("bork");
                                // conn.flush();
                            },
                            _ => {
                                warn!("xcb: unknown event {}", response_type);
                            },
                        }
                    },
                    None => {
                        println!("{:#?}", "oh no");
                        break;
                    }
                }
            }

        }
        else {
            error!("could not connect to X server!");
        }
    });

    // gtk::timeout_add(10, clone!(wm_util move || {
    //     if let Ok(msg_result) = rx.try_recv() {
    //         match msg_result {
    //             Ok(msg) => {
    //                 if let Ok(msg) = msg {
    //                     if msg.starts_with("W") {
    //                         let workspaces = bsp::parse_workspaces(msg);
    //                         wm_util.emit_value(
    //                             Event::Workspace,
    //                             EventValue::Workspaces(workspaces),
    //                         );
    //                     }
    //                 }
    //             },
    //             Err(err) => {
    //                 warn!("{}, restarting thread", err.to_lowercase());
    //                 gtk::timeout_add(100, clone!(wm_util move || {
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
