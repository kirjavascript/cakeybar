use gtk;
use wm::ipc;
use wm::ipc::commands::*;
use wm::ipc::exec::run_command;
use wm::WMUtil;

use crossbeam_channel as channel;
use std::fs::remove_file;
use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::thread;

pub fn listen(wm_util: &WMUtil) {
    let socket_path = ipc::get_socket_path();
    // remove from last time
    remove_file(&socket_path).ok();

    // start listening
    let (s, r) = channel::unbounded();
    thread::spawn(move || match UnixListener::bind(&socket_path) {
        Ok(listener) => {
            for connection in listener.incoming() {
                match connection {
                    Ok(stream) => {
                        thread::spawn(clone!(s || handle_stream(stream, s)));
                    }
                    Err(err) => {
                        error!("IPC connection error: {}", err);
                    }
                }
            }
        }
        Err(err) => error!("Cannot start IPC {}", err),
    });

    // receive events
    gtk::timeout_add(10, clone!(wm_util move || {
        if let Some((input, cmd)) = r.try_recv() {
            info!("received {:?} via IPC...", input);
            run_command(&wm_util, cmd);
        }
        gtk::Continue(true)
    }));
}

fn handle_stream(mut stream: UnixStream, s: channel::Sender<(String, Command)>) {
    let mut buf = [0; 256];
    stream.read(&mut buf).ok();
    // convert to string
    let input = buf
        .iter()
        .filter(|c| **c != 0)
        .map(|c| *c as char)
        .collect::<String>();

    match ipc::parser::parse_message(&input) {
        Ok(cmd) => {
            // send IPC response
            stream.write(format!("{}", cmd).as_bytes()).ok();
            // send to main thread
            s.send((input, cmd));
        }
        Err(_err) => {
            stream
                .write(format!("e:cannot parse {:?}", input).as_bytes())
                .ok();
        }
    }
}
