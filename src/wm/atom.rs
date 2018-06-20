use std::collections::HashMap;
use std::cell::RefCell;
use xcb;

macro_rules! atoms {
    ( $( $x:ident ),* ) => {
        #[allow(non_snake_case)]
        $(pub const $x: &'static str = stringify!($x);)*
    }
}

atoms!(
    _NET_SYSTEM_TRAY_S0,
    _NET_SYSTEM_TRAY_ORIENTATION,
    _NET_SYSTEM_TRAY_OPCODE,
    _NET_SYSTEM_TRAY_VISUAL,
    _NET_WM_WINDOW_TYPE,
    _NET_WM_WINDOW_TYPE_NORMAL,
    _NET_WM_WINDOW_TYPE_DOCK,
    _NET_ACTIVE_WINDOW,
    _NET_WM_STATE,
    _NET_WM_STATE_SKIP_TASKBAR,
    _NET_WM_STATE_STICKY,
    _NET_WM_STATE_FULLSCREEN,
    _NET_STARTUP_ID,
    _NET_WM_STRUT,
    _NET_WM_STRUT_PARTIAL,
    _MOTIF_WM_HINTS,
    WM_TAKE_FOCUS,
    WM_DELETE_WINDOW,
    WM_PROTOCOLS,
    WM_WINDOW_ROLE,
    _COMPTON_SHADOW,
    MANAGER
);

pub struct Atoms<'a> {
    conn: &'a xcb::Connection,
    cache: RefCell<HashMap<String, xcb::Atom>>
}

impl<'a> Atoms<'a> {
    pub fn new(conn: &xcb::Connection) -> Atoms {
        Atoms {
            conn: conn,
            cache: RefCell::new(HashMap::new())
        }
    }

    pub fn get(&self, name: &str) -> xcb::Atom {
        let mut cache = self.cache.borrow_mut();
        if cache.contains_key(name) {
            *cache.get(name).unwrap()
        }
        else {
            let atom = xcb::intern_atom(self.conn, false, name).get_reply().unwrap().atom();
            cache.insert(name.to_string(), atom);
            atom
        }
    }
}
