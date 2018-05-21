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
use self::i3ipc::event::Event;

use std::thread;
use std::sync::mpsc;

pub struct I3Workspace { }

// better as a macro?
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
        // misc config
        let show_all = config.get_bool_or("show_all", false);
        let show_name = config.get_bool_or("show_name", false);

        // attach wrapper
        let wrapper = Box::new(Orientation::Horizontal, spacing);
        I3Workspace::init_widget(&wrapper, config);
        container.add(&wrapper);

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

        let mut labels: Vec<(Label, &Workspace)> = Vec::new();

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
            labels.push((label, workspace));
        });

        // listen for workspace events in another thread



    }
}

        // let (tx, rx) = mpsc::channel();

        // thread::spawn(move || {
        //     let mut listener = I3EventListener::connect().unwrap();
        //     let subs = [Subscription::Workspace];
        //     listener.subscribe(&subs).unwrap();

        //     for event in listener.listen() {
        //         let _ = match event.unwrap() {
        //             Event::WorkspaceEvent(e) => tx.send(e),
        //             _ => unreachable!()
        //         };
        //     }
        // });

        // gtk::timeout_add(10, move || {
        //     if let Ok(msg) = rx.try_recv() {
        //         // println!("{:#?}", msg.current.unwrap().name);
        //         // match msg.change {

        //         // }
        //     }
        //     gtk::Continue(true)
        // });

        // legacy


        // let mut labels: Vec<Label> = Vec::new();

        // gtk::timeout_add(100, move || {

        //     if let Ok(workspaces) = connection.get_workspaces() {
        //         // get workspaces for this window
        //         let mut filtered: Vec<&Workspace> = workspaces.workspaces.iter()
        //             .filter(|x| {
        //                 if !show_all && has_name {
        //                     x.output == monitor_name
        //                 } else {
        //                     true
        //                 }
        //             })
        //             .collect();
        //         // sort by num
        //         filtered.sort_by(|a, b| a.num.cmp(&b.num));
        //         let filtered = filtered;

        //         for (i, workspace) in filtered.iter().enumerate() {
        //             let added_new = if let Some(label) = labels.get_mut(i) {
        //                 // if a label exists
        //                 if show_name {
        //                     label.set_label(&workspace.name);
        //                 } else {
        //                     label.set_label(&workspace.num.to_string());
        //                 };
        //                 if let Some(ctx) = label.get_style_context() {
        //                     if workspace.focused {
        //                         StyleContextExt::add_class(&ctx, "focused");
        //                     } else {
        //                         StyleContextExt::remove_class(&ctx, "focused");
        //                     }
        //                     if workspace.visible {
        //                         StyleContextExt::add_class(&ctx, "visible");
        //                     } else {
        //                         StyleContextExt::remove_class(&ctx, "visible");
        //                     }
        //                 }
        //                 None
        //             }
        //             else {
        //                 // otherwise, create a new one
        //                 let label = Label::new(None);
        //                 // label.set_label(&workspace.name);
        //                 wrapper.add(&label);
        //                 wrapper.show_all();
        //                 Some(label)
        //             };
        //             if let Some(added) = added_new {
        //                 // if we created a new one, add it to the vec
        //                 labels.push(added);
        //             }
        //         }
        //         // remove extra items
        //         let work_len = filtered.len();
        //         let label_len = labels.len();
        //         if label_len > work_len {
        //             labels.splice(work_len.., vec![]).for_each(|w| {
        //                 wrapper.remove(&w);
        //             });
        //         }

        //     }
        //     gtk::Continue(true)
        // });
