extern crate i3ipc;

use super::{Component, Bar, gtk, ComponentConfig};
use gtk::prelude::*;
use gtk::{Label, Box, Orientation, EventBox, LabelExt, Button, WidgetExt, StyleContextExt};
use gdk;
use gdk::{Screen, ScreenExt, Rectangle};

use self::i3ipc::I3Connection;
use self::i3ipc::reply::{Workspace, Workspaces};
use self::i3ipc::I3EventListener;
use self::i3ipc::Subscription;
use self::i3ipc::event::{Event};
use self::i3ipc::event::inner::WorkspaceChange;

use std::thread;
use std::sync::mpsc;

pub struct I3Workspace { }

fn get_set_class(ctx: gtk::StyleContext) -> impl Fn(&str, bool) {
    move |s, b| {
        if b { StyleContextExt::add_class(&ctx, s); }
        else { StyleContextExt::remove_class(&ctx, s); }
    }
}

impl Component for I3Workspace {
    fn init(container: &gtk::Box, config: &ComponentConfig, bar: &Bar){

        // get spacing
        let spacing = config.get_int_or("spacing", 0) as i32;

        // attach wrapper
        let wrapper = Box::new(Orientation::Horizontal, spacing);
        I3Workspace::init_widget(&wrapper, config);
        container.add(&wrapper);

        // load thread
        I3Workspace::load_thread(&wrapper, &config, &bar);
    }
}

impl I3Workspace {
    fn load_thread(wrapper: &gtk::Box, config: &ComponentConfig, bar: &Bar) {

        // misc config
        let show_all = config.get_bool_or("show_all", false);
        let show_name = config.get_bool_or("show_name", false);

        // get monitor name / details
        let screen = Screen::get_default().unwrap();
        let monitor_index = bar.config.monitor_index as i32;
        let monitor_name_opt = screen.get_monitor_plug_name(monitor_index);
        let has_name = monitor_name_opt.is_some();
        let monitor_name = monitor_name_opt.unwrap_or("poop".to_string());

        // i3 connection
        let mut connection = I3Connection::connect().unwrap();

        // get initial workspace list
        let workspace_list = connection.get_workspaces()
            .unwrap_or(Workspaces { workspaces: Vec::new()})
            .workspaces;

        let mut workspaces: Vec<&Workspace> = workspace_list
            .iter()
            .filter(|w| {
                if !show_all && has_name {
                    w.output == monitor_name
                } else {
                    true
                }
            })
        .collect();

        // sort by number
        workspaces.sort_by(|a, b| a.num.cmp(&b.num));

        println!("{:#?}", workspaces);

        // create initial UI

        // let mut labels: Vec<(Label, &Workspace)> = Vec::new();

        workspaces.iter().for_each(|workspace| {
            let label = Label::new(None);
            if show_name {
                label.set_label(&workspace.name);
            } else {
                label.set_label(&workspace.num.to_string());
            };
            if let Some(ctx) = label.get_style_context() {
                let set_class = get_set_class(ctx);
                set_class("focused", workspace.focused);
                set_class("visible", workspace.visible);
                set_class("urgent", workspace.urgent);
            }
            // style
            wrapper.add(&label);
            wrapper.show_all();
            // labels.push((label, workspace));
        });

        // listen for workspace events in another thread

        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let listener_result = I3EventListener::connect();
            match listener_result {
                Ok(mut listener) => {
                    let subs = [Subscription::Workspace];
                    listener.subscribe(&subs).unwrap();

                    for event in listener.listen() {
                        match event {
                            Ok(message) => {
                                match message {
                                    Event::WorkspaceEvent(e) => tx.send(Ok(e)),
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

        let mut labels: Vec<Label> = Vec::new();

        let wrapper_clone = wrapper.clone();
        gtk::timeout_add(10, move || {
            if let Ok(msg_result) = rx.try_recv() {
                match msg_result {
                    Ok(msg) => {
                        // TODO: update UI by diffing for better perf
                        // msg.change = WorkspaceChange
                        // Focus Init Empty Urgent Rename Reload Restored Move Unknown

                        // remove old children
                        wrapper_clone.get_children().iter().for_each(|w| {
                            wrapper_clone.remove(w);
                        });

                        // get initial workspace list
                        let workspace_list = connection.get_workspaces()
                            .unwrap_or(Workspaces { workspaces: Vec::new()})
                            .workspaces;

                        let mut workspaces: Vec<&Workspace> = workspace_list
                            .iter()
                            .filter(|w| {
                                if !show_all && has_name {
                                    w.output == monitor_name
                                } else {
                                    true
                                }
                            })
                        .collect();

                        workspaces.iter().for_each(|workspace| {
                            let label = Label::new(None);
                            if show_name {
                                label.set_label(&workspace.name);
                            } else {
                                label.set_label(&workspace.num.to_string());
                            };
                            if let Some(ctx) = label.get_style_context() {
                                let set_class = get_set_class(ctx);
                                set_class("focused", workspace.focused);
                                set_class("visible", workspace.visible);
                                set_class("urgent", workspace.urgent);
                            }
                            // style
                            wrapper_clone.add(&label);
                            wrapper_clone.show_all();
                        });

                        // sort by number
                        workspaces.sort_by(|a, b| a.num.cmp(&b.num));
                    },
                    Err(err) => {
                        eprintln!("{}\nTODO: restart thread", err);

                        // I3Workspace::load_thread(&wrapper_clone, &config, &bar);
                        return gtk::Continue(false);
                    },
                };
            }
            gtk::Continue(true)
        });
    }
}
