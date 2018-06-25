use gtk;
use i3ipc::I3EventListener;
use i3ipc::Subscription;
use i3ipc::event::Event as I3Event;
use wm::events::Event;

use std::thread;
use std::sync::mpsc;

use futures::Future;
use parallel_event_emitter::*;

pub fn listen(wm_util: &::wm::WMUtil) {

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

    gtk::timeout_add(10, clone!(wm_util move || {
        if let Ok(msg_result) = rx.try_recv() {
            match msg_result {
                Ok(msg) => {
                    wm_util.data
                        .borrow_mut()
                        .events
                        .emit_value(Event::Mode, msg.change)
                        .wait()
                        .unwrap();
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
    }));
}
