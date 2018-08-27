use gtk;
use wm::ipc;

use std::thread;
use std::os::unix::net::{UnixStream, UnixListener};
use std::io::{Read, Write};
use std::fs::remove_file;
use crossbeam_channel as channel;

pub fn listen(_wm_util: &::wm::WMUtil) {
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
            Err(err) => error!("Cannot start IPC {}", err),
        }
    });

    // receive events
    gtk::timeout_add(10, move || {
        if let Some(msg) = r.try_recv() {
            info!("IPC {:#?}", msg);
        }
        gtk::Continue(true)
    });
}

fn handle_stream(mut stream: UnixStream, s: channel::Sender<String>) {
    let mut buf = [0; 256];
    stream.read(&mut buf).ok();
    // convert to string
    let input = buf.iter()
        .filter(|c| **c != 0)
        .map(|c| *c as char)
        .collect::<String>();
    // send to main thread
    s.send(input);
    // send IPC response
    stream.write("RESPONSE".as_bytes()).ok();
}
