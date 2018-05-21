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

        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let mut listener = I3EventListener::connect().unwrap();
            let subs = [Subscription::Window];
            listener.subscribe(&subs).unwrap();

            for event in listener.listen() {
                match event {
                    Ok(message) => {
                        match message {
                            Event::WindowEvent(e) => tx.send(e),
                            _ => unreachable!(),
                        };
                    },
                    Err(err) => {
                        println!("{:#?}", err);
                        break;
                    },
                };
            }
            eprintln!("TODO: restart i3ipc");

            // send message that the thread is dead - start a new one (gtk::COntinue(false))
            illegal
        });

        let label_clone = label.clone();
        gtk::timeout_add(10, move || {
            if let Ok(msg) = rx.try_recv() {
                label_clone.set_text(&msg.container.name.unwrap_or("".to_owned()));
            }
            gtk::Continue(true)
        });


    }
}
