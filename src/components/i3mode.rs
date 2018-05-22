use super::{Component, Bar, gtk, ComponentConfig};
use gtk::prelude::*;
use gtk::{Label};

use i3ipc::I3EventListener;
use i3ipc::Subscription;
use i3ipc::event::Event;

use std::thread;
use std::sync::mpsc;

pub struct I3Mode { }

impl Component for I3Mode {
    fn init(container: &gtk::Box, config: &ComponentConfig, _bar: &Bar){
        let label = Label::new(None);
        I3Mode::init_widget(&label, config);
        container.add(&label);
        I3Mode::load_thread(&label);
    }
}

#[allow(unused_must_use)]
impl I3Mode {
    fn load_thread(label: &Label) {
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
                                    Event::ModeEvent(e) => tx.send(Ok(e)),
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

        let label_clone = label.clone();
        let tick = move || {
            if let Ok(msg_result) = rx.try_recv() {
                match msg_result {
                    Ok(msg) => {
                        let is_default = msg.change == "default";

                        if is_default {
                            label_clone.hide();
                        } else {
                            label_clone.show();
                            label_clone.set_text(&msg.change);
                        }
                    },
                    Err(err) => {
                        eprintln!("{}, restarting thread", err);
                        let label_clone_clone = label_clone.clone();
                        gtk::timeout_add(100, move || {
                            I3Mode::load_thread(&label_clone_clone);
                            gtk::Continue(false)
                        });
                        return gtk::Continue(false);
                    },
                };
            }
            gtk::Continue(true)
        };

        gtk::timeout_add(10, tick);
    }
}
