// much inspiration comes from https://github.com/thatsmydoing/rusttray

use chan;
use chan_signal;
use xcb;
use wm;

pub mod ipc;
pub mod manager;

use std::thread;
use std::sync::Arc;
use std::process::Command;
use std::env::current_exe;

const EXIT_FAILED_CONNECT: i32 = 10;
const EXIT_FAILED_SELECT: i32 = 11;

pub fn as_subprocess() {
    if let Ok(path) = current_exe() {
        Command::new(path)
            .arg("-t")
            .spawn()
            .expect("failed to launch tray");
    }
}

pub fn main() -> i32 {
    let signal = chan_signal::notify(&[
        chan_signal::Signal::INT,
        chan_signal::Signal::TERM,
        chan_signal::Signal::KILL,
    ]);

    if let Ok((conn, screen_num)) = xcb::Connection::connect(None) {
        let conn = Arc::new(conn);
        let atoms = wm::atom::Atoms::new(&conn);
        let screen_num = screen_num as usize;

        let (tx_ipc, rx_ipc) = ipc::get_client();

        let setup = conn.get_setup();
        let screen = setup.roots().nth(screen_num).unwrap();

        let mut manager = manager::Manager::new(&conn, &atoms, &screen, tx_ipc);

        if !manager.is_selection_available() {
            warn!("another system tray is already running");
            return EXIT_FAILED_SELECT
        }

        let (tx, rx) = chan::sync(0);
        thread::spawn(clone!(conn move || {
            loop {
                match conn.wait_for_event() {
                    Some(event) => { tx.send(event); },
                    None => { break; }
                }
            }
        }));

        manager.create();

        let fullscreen_tick = chan::tick_ms(100);

        loop {
            chan_select!(
                rx_ipc.recv() -> ipc_opt => {
                    if let Some(msg) = ipc_opt {
                        manager.handle_ipc_message(msg);
                    }
                },
                rx.recv() -> event_opt => {
                    if let Some(event) = event_opt {
                        if let Some(code) = manager.handle_event(event) {
                            info!("system tray exited with code {}", code);
                            return code
                        }
                    } else {
                        error!("X connection is rip - killed by XKillClient(), maybe?");
                    }
                },
                signal.recv() => {
                    manager.finish();
                },
                fullscreen_tick.recv() => {
                    if wm::xcb::check_fullscreen(&conn, &atoms, &screen) {
                        manager.hide();
                    } else {
                        manager.show();
                    }
                },
            );
        }
    }
    else {
        error!("could not connect to X server!");
        return EXIT_FAILED_CONNECT
    }
}
