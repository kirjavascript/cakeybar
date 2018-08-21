use wm::ipc;

use std::thread;
use std::os::unix::net::{UnixStream, UnixListener};
use std::io::{Read, Write};
use std::fs::remove_file;
use std::sync::mpsc;

pub fn listen(wm_util: &::wm::WMUtil) {
    let socket_path = ipc::get_socket_path();
    // remove from last time
    remove_file(&socket_path).ok();

    // thread::spawn(move || {

    // });
    // let (tx, rx) = mpsc::channel();
    // match UnixListener::bind(&socket_path) {
    //     Ok(listener) => {
    //         for connection in listener.incoming() {
    //             match connection {
    //                 Ok(stream) => {
    //                     thread::spawn(move || handle_stream(stream, tx));
    //                 },
    //                 Err(err) => {
    //                     error!("IPC connection error: {}", err);
    //                 }
    //             }
    //         }
    //     },
    //     Err(err) => error!("Cannot start IPC"),
    // }
    println!("{{:#?}}");
}

fn handle_stream(stream: UnixStream, tx: mpsc::Sender<()>) {

}
