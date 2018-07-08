// use gtk;

use std::thread;
// use std::sync::mpsc;

// use std::os::unix::net::{UnixStream};
use std::io::{Read}; // Error, Write,

use wm::bsp;

pub fn listen(_wm_util: &::wm::WMUtil) {

    // let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        match bsp::connect() {
            Ok(mut stream) => {
                bsp::write_message(&mut stream, "subscribe report".to_string()).ok();

                let mut current = [0; 1];
                let mut msg: Vec<u8> = Vec::new();
                loop {
                    if let Ok(_) = stream.read(&mut current) {
                        if current[0] == 10 {
                            info!("TODO: {:?}", String::from_utf8(msg.clone()));
                            msg.clear();
                        } else {
                            msg.push(current[0]);
                        }
                    }
                }
            },
            Err(e) => {
                error!("{:?}", e);
            },
        }
    });

    // gtk::timeout_add(10, clone!(wm_util move || {
    //     if let Ok(msg_result) = rx.try_recv() {
    //         match msg_result {
    //             Ok(msg) => {
    //                 match msg {
    //                 }
    //             },
    //             Err(err) => {
    //                 warn!("{}, restarting thread", err.to_lowercase());
    //                 gtk::timeout_add(100, clone!(wm_util move || {
    //                     listen(&wm_util);
    //                     gtk::Continue(false)
    //                 }));
    //                 return gtk::Continue(false);
    //             },
    //         };
    //     }
    //     gtk::Continue(true)
    // }));
}
