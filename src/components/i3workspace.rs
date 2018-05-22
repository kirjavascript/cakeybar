
use super::{Component, Bar, gtk, ComponentConfig};
use gtk::prelude::*;
use gtk::{Label, Box, Orientation, LabelExt, WidgetExt, StyleContextExt};
use gdk::{Screen, ScreenExt};

use i3ipc::{I3Connection, I3EventListener, Subscription};
use i3ipc::reply::{Workspace, Workspaces};
use i3ipc::event::{Event};
// use self::i3ipc::event::inner::WorkspaceChange;

use std::thread;
use std::sync::mpsc;

pub struct I3Workspace { }

impl Component for I3Workspace {
    fn init(container: &gtk::Box, config: &ComponentConfig, bar: &Bar){

        // get spacing
        let spacing = config.get_int_or("spacing", 0) as i32;

        // misc config
        let show_all = config.get_bool_or("show_all", false);
        let show_name = config.get_bool_or("show_name", false);

        // attach wrapper
        let wrapper = Box::new(Orientation::Horizontal, spacing);
        I3Workspace::init_widget(&wrapper, config);
        container.add(&wrapper);

        // load thread
        I3Workspace::load_thread(&wrapper, show_name, show_all, bar.config.monitor_index as i32);
    }
}

#[allow(unused_must_use)]
impl I3Workspace {
    fn load_thread(
        wrapper: &gtk::Box,
        show_name: bool,
        show_all: bool,
        monitor_index: i32,
    ) {
        // get monitor name / details
        let screen = Screen::get_default().unwrap();
        let monitor_name_opt = screen.get_monitor_plug_name(monitor_index);
        let has_name = monitor_name_opt.is_some();
        let monitor_name = monitor_name_opt.unwrap_or("poop".to_string());

        // i3 connection
        let connection_result = I3Connection::connect();
        match connection_result {
            Ok(mut connection) => {

                // remove children of widget from previous thread

                for child in wrapper.get_children().iter() {
                    wrapper.remove(child);
                }

                let workspace_list = get_workspace_list(&mut connection);
                let workspaces = get_workspaces(&workspace_list, show_all, has_name, monitor_name.clone());

                // create initial UI

                let mut labels: Vec<Label> = Vec::new();

                for workspace in workspaces.iter() {
                    let label = Label::new(None);
                    set_label_attrs(&label, &workspace, show_name);
                    wrapper.add(&label);
                    labels.push(label);
                }
                wrapper.show_all();

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


                let wrapper_clone = wrapper.clone();
                gtk::timeout_add(10, move || {
                    if let Ok(msg_result) = rx.try_recv() {
                        match msg_result {
                            Ok(_msg) => {
                                // TODO: update UI by diffing for better perf
                                // msg.change = WorkspaceChange
                                // Focus Init Empty Urgent Rename Reload Restored Move Unknown

                                let workspace_list = get_workspace_list(&mut connection);
                                let workspaces = get_workspaces(&workspace_list, show_all, has_name, monitor_name.clone());

                                for (i, workspace) in workspaces.iter().enumerate() {
                                    let added_new = if let Some(label) = labels.get_mut(i) {
                                        set_label_attrs(&label, &workspace, show_name);
                                        None
                                    } else {
                                        let label = Label::new(None);
                                        set_label_attrs(&label, &workspace, show_name);
                                        wrapper_clone.add(&label);
                                        Some(label)
                                    };
                                    if let Some(added) = added_new {
                                        labels.push(added);
                                    }
                                }
                                wrapper_clone.show_all();

                                // remove items
                                let work_len = workspaces.len();
                                let label_len = labels.len();
                                if label_len > work_len {
                                    labels.splice(work_len.., vec![]).for_each(|w| {
                                        wrapper_clone.remove(&w);
                                    });
                                }

                            },
                            Err(err) => {
                                // thread has stopped
                                handle_err(err, &wrapper_clone, show_name, show_all, monitor_index);
                                return gtk::Continue(false);
                            },
                        };
                    }
                    gtk::Continue(true)
                });
            },
            Err(err) => {
                // connection failed
                let err_str = format!("{}", err);
                handle_err(err_str, wrapper, show_name, show_all, monitor_index);
            },
        };

    }
}

fn handle_err(err: String, wrapper: &gtk::Box, show_name: bool, show_all: bool, monitor_index: i32) {
    eprintln!("{}, restarting thread", err);
    let wrapper_clone = wrapper.clone();
    gtk::timeout_add(100, move || {
        I3Workspace::load_thread(&wrapper_clone, show_name, show_all, monitor_index);
        gtk::Continue(false)
    });
}

fn get_workspace_list(connection: &mut I3Connection) -> Vec<Workspace> {
    connection.get_workspaces()
        .unwrap_or(Workspaces { workspaces: Vec::new()})
        .workspaces
}

fn get_workspaces<'a>(workspace_list: &'a Vec<Workspace>, show_all: bool, has_name: bool, monitor_name: String) -> Vec<&'a Workspace> {
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
    workspaces
}

fn get_set_class(ctx: gtk::StyleContext) -> impl Fn(&str, bool) {
    move |s, b| {
        if b { StyleContextExt::add_class(&ctx, s); }
        else { StyleContextExt::remove_class(&ctx, s); }
    }
}


fn set_label_attrs(label: &Label, workspace: &Workspace, show_name: bool) {
    if show_name {
        label.set_label(&workspace.name);
    } else {
        label.set_label(&workspace.num.to_string());
    };
    // style
    if let Some(ctx) = label.get_style_context() {
        let set_class = get_set_class(ctx);
        set_class("focused", workspace.focused);
        set_class("visible", workspace.visible);
        set_class("urgent", workspace.urgent);
    }
}
