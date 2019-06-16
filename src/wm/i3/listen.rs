use gtk;
use i3ipc::event::Event as I3Event;
use i3ipc::{I3EventListener, Subscription};
use crate::wm::events::{Event, EventValue};
use crate::wm::i3;
use crate::wm::workspace::Workspace;

use std::sync::mpsc;
use std::thread;

enum I3Msg {
    Mode(String),
    Workspace(Vec<Workspace>),
}

pub fn listen(wm_util: &crate::wm::WMUtil) {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let listener_result = I3EventListener::connect();
        match listener_result {
            Ok(mut listener) => {
                let subs = [Subscription::Mode, Subscription::Workspace];
                listener.subscribe(&subs).unwrap();

                let mut connection = i3::connect();

                for event in listener.listen() {
                    match event {
                        Ok(message) => {
                            match message {
                                I3Event::ModeEvent(e) => {
                                    tx.send(Ok(I3Msg::Mode(e.change))).ok();
                                }
                                I3Event::WorkspaceEvent(_e) => {
                                    // Focus Init Empty Urgent Rename Reload Restored Move Unknown

                                    if let Ok(ref mut connection) = connection {
                                        tx.send(Ok(I3Msg::Workspace(i3::get_workspaces(
                                            connection,
                                        )))).ok();
                                    } else if let Err(ref err) = connection {
                                        error!("{} (try reloading i3 config)", err);
                                    }
                                }
                                _ => unreachable!(),
                            };
                        }
                        Err(err) => {
                            // listener is rip
                            tx.send(Err(format!("{}", err))).unwrap();
                            break;
                        }
                    };
                }
            }
            Err(err) => {
                // socket failed to connect
                tx.send(Err(format!("{}", err))).unwrap();
            }
        };
    });

    gtk::timeout_add(10, clone!(wm_util move || {
        if let Ok(msg_result) = rx.try_recv() {
            match msg_result {
                Ok(msg) => {
                    match msg {
                        I3Msg::Mode(value) => {
                            wm_util.emit_value(
                                Event::Mode,
                                EventValue::String(value),
                            );
                        },
                        I3Msg::Workspace(value) => {
                            wm_util.emit_value(
                                Event::Workspace,
                                EventValue::Workspaces(value),
                            );
                        },
                    }
                },
                Err(err) => {
                    warn!("{}, restarting thread", err.to_lowercase());
                    gtk::timeout_add(1000, clone!(wm_util move || {
                        listen(&wm_util);
                        gtk::Continue(false)
                    }));
                    return gtk::Continue(false);
                },
            };
        }
        gtk::Continue(true)
    }));
}
