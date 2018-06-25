use super::{Component, Bar, gtk, ComponentConfig};
use gtk::prelude::*;
use gtk::{Label, Box, EventBox, Orientation, LabelExt, WidgetExt, StyleContextExt};
use gdk::{Screen, ScreenExt};

use i3ipc::{I3Connection, I3EventListener, Subscription};
use i3ipc::reply::{Workspace, Workspaces};
// use i3ipc::event::{Event};
use wm;
use wm::events::Event;

use std::thread;
use std::sync::mpsc;

pub struct I3Workspace { }

// Workspaces

impl Component for I3Workspace {
    fn init(container: &Box, config: &ComponentConfig, bar: &Bar){
        if bar.wm_util.get_wm_type() != wm::WMType::I3 {
            return
        }

        let monitor_index = bar.config.get_int_or("monitor", 0) as i32;

        // get spacing
        let spacing = config.get_int_or("spacing", 0) as i32;

        // misc config
        let show_all = config.get_bool_or("show_all", false);
        let show_name = config.get_bool_or("show_name", false);

        // attach wrapper
        let wrapper = Box::new(Orientation::Horizontal, spacing);
        Self::init_widget(&wrapper, config);

        // add to container and show
        container.add(&wrapper);
        wrapper.show();

        // load thread
        // Self::load_thread(&wrapper, show_name, show_all, monitor_index);

        let (has_name, monitor_name) = get_monitor_name(monitor_index);


        let mut connection = I3Connection::connect().unwrap();
        let workspace_list = get_workspace_list(&mut connection);
        let workspaces = get_workspaces(&workspace_list, show_all, has_name, monitor_name.clone());

        // create initial UI

        let mut labels: Vec<Label> = Vec::new();

        for workspace in workspaces.iter() {
            let label = Label::new(None);
            set_label_attrs(&label, &workspace, show_name);
            let ebox = add_event_box(&label, workspace.name.clone());
            wrapper.add(&ebox);
            labels.push(label);
        }
        wrapper.show_all();

        // listen for events
        bar.wm_util.data.borrow_mut()
            .events.add_listener(Event::Workspace, clone!(wrapper
                move || {
                    let mut connection = I3Connection::connect().unwrap();
                    let workspace_list = get_workspace_list(&mut connection);
                    let workspaces = get_workspaces(&workspace_list, show_all, has_name, monitor_name.clone());

                    for (i, workspace) in workspaces.iter().enumerate() {
                        let added_new = if let Some(label) = labels.get_mut(i) {
                            // if a label already exists
                            set_label_attrs(&label, &workspace, show_name);
                            None
                        } else {
                            // if adding a new label
                            let label = Label::new(None);
                            set_label_attrs(&label, &workspace, show_name);
                            let ebox = add_event_box(&label, workspace.name.clone());
                            wrapper.add(&ebox);
                            Some(label)
                        };
                        if let Some(added) = added_new {
                            labels.push(added);
                        }
                    }
                    wrapper.show_all();

                    // remove items
                    let work_len = workspaces.len();
                    let label_len = labels.len();
                    if label_len > work_len {
                        labels.splice(work_len.., vec![]).for_each(|w| {
                            if let Some(parent) = w.get_parent() {
                                // nuke the event box
                                parent.destroy();
                            }
                        });
                    }

                    Ok(())
                }
            )).unwrap();

    }
}


// i3 stuff

pub fn run_command(string: &str) {
    let connection_result = I3Connection::connect();
    match connection_result {
        Ok(mut connection) => {
            connection.run_command(string).ok();
        },
        Err(err) => {
            error!("running i3 command {}", err);
        },
    }
}

pub fn scroll_workspace(is_next: bool, monitor_index: i32) {
    let connection_result = I3Connection::connect();
    match connection_result {
        Ok(mut connection) => {

            // get monitor name / details
            let (has_name, monitor_name) = get_monitor_name(monitor_index);

            // get workspace details
            let workspace_list = get_workspace_list(&mut connection);
            let mut workspaces = get_workspaces(&workspace_list, false, has_name, monitor_name.clone());
            // so we can search backwards
            if !is_next {
                workspaces.reverse();
            }

            // get focused workspace
            let focused_opt = workspaces.iter().find(|x| x.focused);
            if let Some(focused) = focused_opt {
                // get next one
                let next_opt = workspaces.iter().find(|x| {
                    if is_next {
                        x.num > focused.num
                    } else {
                        x.num < focused.num
                    }
                });
                if let Some(next) = next_opt {
                    let command = format!("workspace {}", next.name);
                    connection.run_command(&command)
                        .expect("something went wrong running an i3 command");
                }
            }
        },
        Err(err) => {
            error!("getting i3 connection {}", err);
        },
    }
}

fn get_monitor_name(monitor_index: i32) -> (bool, String) {
    let screen = Screen::get_default().unwrap();
    let monitor_name_opt = screen.get_monitor_plug_name(monitor_index);
    let has_name = monitor_name_opt.is_some();
    let monitor_name = monitor_name_opt.unwrap_or("poop".to_string());
    (has_name, monitor_name)
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

// UI stuff

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

fn add_event_box(label: &Label, workspace_name: String) -> EventBox {
    let ebox = EventBox::new();
    ebox.add(label);
    ebox.connect_button_press_event(move |_, _| {
        let command = format!("workspace {}", workspace_name);
        run_command(&command);
        Inhibit(false)
    });
    ebox
}
