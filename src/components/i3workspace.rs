extern crate i3ipc;

use super::{Component, Bar, gtk, ComponentConfig};
use gtk::prelude::*;
use gtk::{Label};

use self::i3ipc::I3EventListener;
use self::i3ipc::Subscription;
use self::i3ipc::event::Event;

use std::thread;
use std::sync::mpsc;

pub struct I3Workspace {
}

impl Component for I3Workspace {
    fn init(container: &gtk::Box, config: &ComponentConfig, _bar: &Bar) {
        let label = Label::new(None);
        WidgetExt::set_name(&label, &config.name);
        container.add(&label);

        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let mut listener = I3EventListener::connect().unwrap();
            let subs = [Subscription::Workspace];
            listener.subscribe(&subs).unwrap();

            for event in listener.listen() {
                let _ = match event.unwrap() {
                    Event::WorkspaceEvent(e) => tx.send(format!("{:?}", e)),
                    _ => unreachable!(),
                };
            }
        });

        let label_clone = label.clone();
        gtk::idle_add(move || {
            if let Ok(msg) = rx.try_recv() {
                label_clone.set_text(&msg);
            }
            gtk::Continue(true)
        });

    }
}
