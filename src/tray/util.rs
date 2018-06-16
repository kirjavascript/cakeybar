use super::{xcb, atom};

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
                    if value.len() > 0 && value[0] == fullscreen_atom {
                        return true;
                    } else {
                        return false;
                    }
                },
                Err(_err) => {},
            }
        },
        Err(_err) => {},
    }
    false
}
