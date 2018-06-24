use gtk;
use i3ipc::I3EventListener;
use i3ipc::Subscription;
use i3ipc::event::Event as I3Event;
use wm::events::Event;

use std::thread;
use std::sync::mpsc;

pub fn listen() {

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let listener_result = I3EventListener::connect();
        match listener_result {
            Ok(mut listener) => {
                let subs = [Subscription::Mode];
                listener.subscribe(&subs).unwrap();

                for event in listener.listen() {
                    match event {
                        Ok(message) => {
                            match message {
                                I3Event::ModeEvent(e) => tx.send(Ok(e)),
                                _ => unreachable!(),
                            };
                        },
                        Err(err) => {
                            // listener is rip
                            tx.send(Err(format!("{}", err)));
                            break;
                        },
                    };
                }
            },
            Err(err) => {
                // socket failed to connect
                tx.send(Err(format!("{}", err)));
            },
        };
    });

    gtk::timeout_add(10, move || {
        if let Ok(msg_result) = rx.try_recv() {
            match msg_result {
                Ok(msg) => {
                    // let is_default = msg.change == "default";

                    // if is_default {
                    //     label.hide();
                    // } else {
                    //     label.show();
                    //     label.set_text(&msg.change);
                    // }
                    // stream.send(&Event::Window(msg.change));
                },
                Err(err) => {
                    info!("{}, restarting thread", err.to_lowercase());
                    gtk::timeout_add(100, move || {
                        // listen(&stream);
                        gtk::Continue(false)
                    });
                    return gtk::Continue(false);
                },
            };
        }
        gtk::Continue(true)
    });
}
