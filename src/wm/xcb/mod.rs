mod listen;

pub use self::listen::listen;

use crate::wm::atom;
use xcb;

pub fn check_fullscreen(conn: &xcb::Connection, atoms: &atom::Atoms, screen: &xcb::Screen) -> bool {
    // get active window
    let cookie = xcb::get_property(
        &conn,
        false,
        screen.root(),
        atoms.get(atom::_NET_ACTIVE_WINDOW),
        xcb::ATOM_WINDOW,
        0,
        8,
    );

    match cookie.get_reply() {
        Ok(reply) => {
            let value: &[u32] = reply.value();
            if value.is_empty() {
                return false;
            }
            // get wm state
            let cookie = xcb::get_property(
                &conn,
                false,
                value[0],
                atoms.get(atom::_NET_WM_STATE),
                xcb::ATOM_ATOM,
                0,
                8,
            );
            match cookie.get_reply() {
                Ok(reply) => {
                    // check if active window is fullscreen
                    let value: &[u32] = reply.value();
                    let fullscreen_atom = atoms.get(atom::_NET_WM_STATE_FULLSCREEN);
                    for v in value.iter() {
                        if *v == fullscreen_atom {
                            return true;
                        }
                    }
                }
                Err(_err) => {}
            }
        }
        Err(_err) => {}
    }
    false
}

pub fn get_string(conn: &xcb::Connection, id: u32, _type: u32, attr: u32) -> String {
    let window: xcb::Window = id;
    let long_length: u32 = 16;
    let mut long_offset: u32 = 0;
    let mut buf = Vec::new();
    loop {
        let cookie = xcb::get_property(
            &conn,
            false,
            window,
            attr,
            _type, // xcb::ATOM_STRING
            long_offset,
            long_length,
        );
        match cookie.get_reply() {
            Ok(reply) => {
                let value: &[u8] = reply.value();
                buf.extend_from_slice(value);
                match reply.bytes_after() {
                    0 => break,
                    _ => {
                        let len = reply.value_len();
                        long_offset += len / 4;
                    }
                }
            }
            Err(err) => {
                error!("{:?}", err);
                break;
            }
        }
    }

    format!("{}", String::from_utf8_lossy(&buf))
}

// debug window order
// if let Ok(reply) = xcb::query_tree(&self.conn, screen.root()).get_reply() {
//     for i in reply.children() {
//         info!("{:#?} {}", get_wm_name(&self.conn, *i), i);
//     }
// }

// use std::sync::Arc;
// pub fn set_strut(window_role: String) {
//     if let Ok((conn, preferred)) = xcb::Connection::connect(None) {
//         let conn = Arc::new(conn);
//         let atoms = atom::Atoms::new(&conn);
//         let preferred = preferred as usize;
//         let setup = conn.get_setup();
//         let screen = setup.roots().nth(preferred).unwrap();
//         let window_role_atom = atoms.get(atom::WM_WINDOW_ROLE);

//         if let Ok(reply) = xcb::query_tree(&conn, screen.root()).get_reply() {
//             // title works...
//             let w = reply.children().iter().find(|w| {
//                 window_role == get_string(&conn, **w, window_role_atom)
//             });
//             conn.flush();
//         }
//     }

// }

//
// adds resize event
// xcb::change_window_attributes(self.conn, self.window, &[
//     (xcb::CW_EVENT_MASK, xcb::EVENT_MASK_STRUCTURE_NOTIFY),
// ]);

// restack window (fixes always on top bug)
// if let Ok(reply) = xcb::query_tree(self.conn, screen.root()).get_reply() {
//     let children = reply.children();
//     let value_mask = (xcb::CONFIG_WINDOW_STACK_MODE | xcb::CONFIG_WINDOW_SIBLING) as u16;
//     // find the first i3 window
//     for child in children {
//         let wm_name = xcb_get_wm_name(self.conn, *child);
//         if wm_name.contains("i3") {
//             // put the window directly above it
//             xcb::configure_window_checked(self.conn, self.window, &[
//                 (value_mask, *child),
//                 (value_mask, xcb::STACK_MODE_ABOVE),
//             ]);
//             break;
//         }
//     }
// }

// attempt to stack gtk bar BELOW tray
// let value_mask = (xcb::CONFIG_WINDOW_STACK_MODE | xcb::CONFIG_WINDOW_SIBLING) as u16;

// if let Ok(reply) = xcb::query_tree(&self.conn, screen.root()).get_reply() {
//     let i3_opt = reply.children().iter().find(|child| {
//          xcb_get_wm_name(&self.conn, **child).contains("i3")
//     });
//     let bar_opt = reply.children().iter().find(|child| {
//         ::NAME == xcb_get_wm_name(&self.conn, **child)
//     });
//     if let Some(bar) = bar_opt {
//         if let Some(i3) = i3_opt {
//             println!("{:#?}", (i3, bar));

//             xcb::configure_window_checked(&self.conn, *i3, &[
//                 (value_mask, *bar),
//                 (value_mask, xcb::STACK_MODE_ABOVE),
//             ]);

//             println!("{:#?}", "swapped");
//             self.conn.flush();
//         }
//     }
// }
