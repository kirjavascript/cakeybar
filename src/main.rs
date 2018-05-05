extern crate xcb;

use std::iter::{Iterator};
macro_rules! set_prop {
    ($conn:expr, $window:expr, $name:expr, @atom $value:expr) => {
        {
            match xcb::intern_atom($conn, true, $value).get_reply() {
                Ok(atom) => set_prop!($conn, $window, $name, &[atom.atom()], "ATOM", 32),
                _ => panic!("Unable to set window property"),
            }
        }
    };
    ($conn:expr, $window:expr, $name:expr, $data:expr) => {
        {
            set_prop!($conn, $window, $name, $data, "CARDINAL", 32)
        }
    };
    ($conn:expr, $window:expr, $name:expr, $data:expr, $type:expr, $size:expr) => {
        {
            let type_atom = xcb::intern_atom($conn, true, $type).get_reply();
            let property = xcb::intern_atom($conn, true, $name).get_reply();
            match (type_atom, property) {
                (Ok(type_atom), Ok(property)) => {
                    let property = property.atom();
                    let type_atom = type_atom.atom();
                    let mode = xcb::PROP_MODE_REPLACE as u8;
                    xcb::change_property($conn, mode, $window, property, type_atom, $size, $data);
                },
                (Err(_), _) | (_, Err(_)) => panic!("Unable to set window property"),
            }
        }
    };
}

extern crate gio;
extern crate gtk;
extern crate chrono;

use gio::prelude::*;
use gtk::prelude::*;
use std::env::args;
use chrono::Local;

use std::thread;


fn current_time() -> String {
    return format!("{}", Local::now().format("%Y-%m-%d %H:%M:%S"));
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::Window::new(gtk::WindowType::Popup);
    application.add_window(&window);

    window.set_title("rustybar");
    window.set_border_width(10);
    window.set_default_size(1920, 40);

    let time = current_time();
    let label = gtk::Label::new(None);
    label.set_text(&time);

    window.add(&label);

    window.show_all();
    window.move_(0, 0);

    let tick = move || {
        let time = current_time();
        label.set_text(&time);
        gtk::Continue(true)
    };

    gtk::timeout_add_seconds(1, tick);
}

fn main() {

    thread::spawn(move || {
        bump();
    });
    let application = gtk::Application::new(
            "com.kirjava.rustybar",
            gio::ApplicationFlags::empty(),
        )
        .expect("Initialization failed...");

    application.connect_startup(|app| {
        build_ui(app);
    });
    application.connect_activate(|_| {});

    application.run(&args().collect::<Vec<_>>());

}

fn bump() {

    let (conn, screen_num) = xcb::Connection::connect(None).unwrap();
    let setup = conn.get_setup();
    let screen = setup.roots().nth(screen_num as usize).unwrap();

    let window = conn.generate_id();

    let values = [
        (xcb::CW_BACK_PIXEL, screen.white_pixel()),
        (xcb::CW_EVENT_MASK, xcb::EVENT_MASK_EXPOSURE | xcb::EVENT_MASK_KEY_PRESS),
    ];

    xcb::create_window(&conn,
        xcb::COPY_FROM_PARENT as u8,
        window,
        screen.root(),
        0, 0,
        150, 40,
        10,
        xcb::WINDOW_CLASS_INPUT_OUTPUT as u16,
        screen.root_visual(),
        &values);
    let title = "Basic Window";
    //
    // Set all window properties
    let start_x = 150 as u32;
    let end_x = 0 + 150 - 1;
    let height = 40;
    let struts = [0, 0, height, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    set_prop!(&conn, window, "_NET_WM_STRUT", &struts[0..4]);
    set_prop!(&conn, window, "_NET_WM_STRUT_PARTIAL", &struts);
    set_prop!(&conn, window, "_NET_WM_WINDOW_TYPE", @atom "_NET_WM_WINDOW_TYPE_DOCK");
    set_prop!(&conn, window, "_NET_WM_STATE", @atom "_NET_WM_STATE_STICKY");
    set_prop!(&conn, window, "_NET_WM_DESKTOP", &[-1]);
    // set_prop!(&conn, window, "_NET_WM_NAME", &title[..3], "UTF8_STRING", 8);
// set_prop!(&conn, window, "WM_NAME", &title[..3], "STRING", 8);

    xcb::map_window(&conn, window);

    // setting title
    xcb::change_property(&conn, xcb::PROP_MODE_REPLACE as u8, window,
            xcb::ATOM_WM_NAME, xcb::ATOM_STRING, 8, title.as_bytes());

    conn.flush();

    // retrieving title
    let cookie = xcb::get_property(&conn, false, window, xcb::ATOM_WM_NAME,
            xcb::ATOM_STRING, 0, 1024);
    if let Ok(reply) = cookie.get_reply() {
        assert_eq!(std::str::from_utf8(reply.value()).unwrap(), title);
    } else {
        panic!("could not retrieve window title!");
    }

    // retrieving a few atoms
    let (wm_state, wm_state_maxv, wm_state_maxh) = {
        let cook = xcb::intern_atom(&conn, true, "_NET_WM_STATE");
        let cook_maxv = xcb::intern_atom(&conn, true, "_NET_WM_STATE_MAXIMIZED_VERT");
        let cook_maxh = xcb::intern_atom(&conn, true, "_NET_WM_STATE_MAXIMIZED_HORZ");

        (cook.get_reply().unwrap().atom(),
            cook_maxv.get_reply().unwrap().atom(),
            cook_maxh.get_reply().unwrap().atom())
    };

    let mut maximized = false;

    loop {
        let event = conn.wait_for_event();
        match event {
            None => { break; }
            Some(event) => {
                let r = event.response_type();
                if r == xcb::KEY_PRESS as u8 {
                    let key_press : &xcb::KeyPressEvent = unsafe {
                        xcb::cast_event(&event)
                    };

                    println!("Key '{}' pressed", key_press.detail());

                    if key_press.detail() == 0x3a { // M (on qwerty)

                        // toggle maximized

                        // ClientMessageData is a memory safe untagged union
                        let data = xcb::ClientMessageData::from_data32([
                            if maximized { 0 } else { 1 },
                            wm_state_maxv, wm_state_maxh,
                            0, 0
                        ]);

                        let ev = xcb::ClientMessageEvent::new(32, window,
                            wm_state, data);

                        xcb::send_event(&conn, false, screen.root(),
                            xcb::EVENT_MASK_STRUCTURE_NOTIFY, &ev);

                        conn.flush();

                        maximized = !maximized;
                    }
                    else if key_press.detail() == 0x18 { // Q (on qwerty)
                        break;
                    }
                }
            }
        }
    }
}
