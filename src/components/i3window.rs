extern crate i3ipc;

use super::{Component, Bar, gtk, ComponentConfig};
use gtk::prelude::*;
use gtk::{Label};

use self::i3ipc::I3EventListener;
use self::i3ipc::Subscription;
use self::i3ipc::event::Event;

use std::thread;
use std::sync::mpsc;

pub struct I3Window { }

impl Component for I3Window {
    fn init(container: &gtk::Box, config: &ComponentConfig, _bar: &Bar){
        let label = Label::new(None);
        I3Window::init_widget(&label, config);
        container.add(&label);
        I3Window::load_thread(&label);
    }

}

impl I3Window {
    fn load_thread(label: &Label) {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let mut listener_result = I3EventListener::connect();
            match listener_result {
                Ok(mut listener) => {
                    let subs = [Subscription::Window];
                    listener.subscribe(&subs).unwrap();

                    for event in listener.listen() {
                        match event {
                            Ok(message) => {
                                match message {
                                    Event::WindowEvent(e) => tx.send(Ok(e)),
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
        gtk::timeout_add(10, move || {
            if let Ok(msg_result) = rx.try_recv() {
                match msg_result {
                    Ok(msg) => {
                        label_clone.set_text(&msg.container.name.unwrap_or("".to_owned()));
                    },
                    Err(err) => {
                        eprintln!("{}, restarting thread", err);
                        let label_clone_clone = label_clone.clone();
                        gtk::timeout_add(100, move || {
                            I3Window::load_thread(&label_clone_clone);
                            gtk::Continue(false)
                        });
                        return gtk::Continue(false);
                    },
                };
            }
            gtk::Continue(true)
        });
    }
}
