use gtk;
use wm::ipc;

use std::thread;
use std::os::unix::net::{UnixStream, UnixListener};
use std::io::{Read, Write};
use std::fs::remove_file;
use crossbeam_channel as channel;

pub fn listen(wm_util: &::wm::WMUtil) {
    let socket_path = ipc::get_socket_path();
    // remove from last time
    remove_file(&socket_path).ok();

    // start listening
    let (s, r) = channel::unbounded();
    thread::spawn(move || {
        match UnixListener::bind(&socket_path) {
            Ok(listener) => {
                for connection in listener.incoming() {
                    match connection {
                        Ok(stream) => {
                            thread::spawn(clone!(s || handle_stream(stream, s)));
                        },
                        Err(err) => {
                            error!("IPC connection error: {}", err);
                        }
                    }
                }
            },
            Err(err) => error!("Cannot start IPC"),
        }
    });

    // receive events
    gtk::timeout_add(10, move || {
        if let Some(msg) = r.try_recv() {
            println!("{:#?}", msg);
        }
        gtk::Continue(true)
    });
}

fn handle_stream(stream: UnixStream, s: channel::Sender<()>) {

}
