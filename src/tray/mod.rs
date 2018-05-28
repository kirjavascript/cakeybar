// initially copied from https://github.com/thatsmydoing/rusttray

// #[macro_use]
// extern crate chan;
// extern crate chan_signal;
// extern crate xcb;

use chan;
use chan_signal;
use xcb;

mod atom;
mod tray;

use std::process;
use std::thread;
use std::sync::Arc;

const PROGRAM: &'static str = "System Tray";
const EXIT_FAILED_CONNECT: i32 = 10;
const EXIT_FAILED_SELECT: i32 = 11;

// fn main() {
//     process::exit(real_main());
// }

pub fn init() -> i32 {
    let signal = chan_signal::notify(&[chan_signal::Signal::INT, chan_signal::Signal::TERM]);

    let size = 20;
    let bg = 0x221122;

    if let Ok((conn, preferred)) = xcb::Connection::connect(None) {
        let conn = Arc::new(conn);
        let atoms = atom::Atoms::new(&conn);

        let mut tray = tray::Tray::new(&conn, &atoms, preferred as usize, size, bg);

        if !tray.is_selection_available() {
            println!("Another system tray is already running");
            return EXIT_FAILED_SELECT
        }

        let (tx, rx) = chan::sync(0);
        {
            let conn = conn.clone();
            thread::spawn(move || {
                loop {
                    match conn.wait_for_event() {
                        Some(event) => { tx.send(event); },
                        None => { break; }
                    }
                }
            });
        }

        tray.create();

        loop {
            chan_select!(
                rx.recv() -> event => {
                    if let Some(code) = tray.handle_event(event.unwrap()) {
                        println!("{:?}", code);
                        return code
                    }
                },
                signal.recv() => {
                    tray.finish();
                }
            );
        }
    }
    else {
        println!("Could not connect to X server!");
        return EXIT_FAILED_CONNECT
    }
}
