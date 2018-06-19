use super::{Component, Bar, gtk, ComponentConfig};
use gtk::prelude::*;
use gtk::{Label};

use i3ipc::I3EventListener;
use i3ipc::Subscription;
use i3ipc::event::Event;

use std::thread;
use std::sync::mpsc;

pub struct I3Window { }

impl Component for I3Window {
    fn init(container: &gtk::Box, config: &ComponentConfig, _bar: &Bar){
        let label = Label::new(None);
        Self::init_widget(&label, config);
        container.add(&label);
        label.show();
        let trunc = config.get_int_or("truncate", 100);
        Self::load_thread(&label, trunc as usize);
    }
}

#[allow(unused_must_use)]
impl I3Window {
    fn load_thread(label: &Label, trunc: usize) {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let listener_result = I3EventListener::connect();
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

        gtk::timeout_add(10, enclose!(label move || {
            if let Ok(msg_result) = rx.try_recv() {
                match msg_result {
                    Ok(msg) => {
                        let name = msg.container.name.clone().unwrap_or("".to_string());
                        let name = if name.len() > trunc {
                            format!("{}...", &name[..trunc])
                        } else {
                            format!("{}", name)
                        };
                        label.set_text(&name);
                    },
                    Err(_err) => {
                        #[cfg(debug_assertions)]
                        eprintln!("{}, restarting thread", _err);
                        gtk::timeout_add(100, enclose!(label move || {
                            Self::load_thread(&label, trunc);
                            gtk::Continue(false)
                        }));
                        return gtk::Continue(false);
                    },
                };
            }
            gtk::Continue(true)
        }));
    }
}
