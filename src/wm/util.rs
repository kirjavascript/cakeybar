use xcb;
use wm::atom;
use std::sync::Arc;

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
                },
                Err(_err) => {},
            }
        },
        Err(_err) => {},
    }
    false
}

pub fn set_strut(window_role: String) {
    if let Ok((conn, preferred)) = xcb::Connection::connect(None) {
        let conn = Arc::new(conn);
        let atoms = atom::Atoms::new(&conn);
        let preferred = preferred as usize;
        let setup = conn.get_setup();
        let screen = setup.roots().nth(preferred).unwrap();
        let window_role_atom = atoms.get(atom::WM_WINDOW_ROLE);

        if let Ok(reply) = xcb::query_tree(&conn, screen.root()).get_reply() {
            // title works...
            let w = reply.children().iter().find(|w| {
                window_role == get_string(&conn, **w, window_role_atom)
            });
            println!("{:#?}", w);
            // for w in reply.children().iter() {
            //     let q = get_string(&conn, *w, window_role_atom);
            //     println!("{:#?}", q);
            // };
            // let bar_opt = reply.children().iter().find(|child| {
            //     ::NAME == wm::util::xcb_get_wm_name(&conn, **child)
            // });
            // xcb::change_property(
            //     conn,
            //     xcb::PROP_MODE_APPEND as u8,
            //     *bar_opt.unwrap(),
            //     atoms.get(tray::atom::_NET_WM_STATE),
            //     xcb::ATOM_ATOM,
            //     32,
            //     &[atoms.get(tray::atom::_net_wm_state_sticky)]
            // );
            conn.flush();
        }
    }

}


#[allow(dead_code)]
pub fn get_string(conn: &xcb::Connection, id: u32, attr: u32) -> String {
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
            xcb::ATOM_STRING,
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
                println!("{:?}", err);
                break;
            }
        }
    }
    let result = String::from_utf8(buf).unwrap();
    let results: Vec<&str> = result.split('\0').collect();
    results[0].to_string()
}
