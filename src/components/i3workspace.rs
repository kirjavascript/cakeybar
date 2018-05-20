extern crate i3ipc;

use super::{Component, Bar, gtk, ComponentConfig};
use gtk::prelude::*;
use gtk::{Label, Box, Orientation, LabelExt, Button, WidgetExt, StyleContextExt};
use gdk::{Screen, ScreenExt, Rectangle};

use self::i3ipc::I3Connection;


pub struct I3Workspace {
}

impl Component for I3Workspace {
    fn init(container: &gtk::Box, config: &ComponentConfig, bar: &Bar){

        // TODO: rewrite with threads and updates for better perf

        // get spacing
        let spacing = config.get_int_or("spacing", 0) as i32;

        // attach wrapper
        let wrapper = Box::new(Orientation::Horizontal, spacing);
        I3Workspace::init_widget(&wrapper, config);
        container.add(&wrapper);

        // i3 connection
        let mut connection = I3Connection::connect().unwrap();

        // get monitor name
        let screen = Screen::get_default().unwrap();
        let monitor_name = screen.get_monitor_plug_name(bar.config.monitor_index as i32);
        let has_name = monitor_name.is_some();
        let monitor_name = monitor_name.unwrap_or("poop".to_string());

        let workspaces = connection.get_workspaces();

        if let Ok(workspaces) = connection.get_workspaces() {
            let test: Vec<_> = workspaces.workspaces.iter().filter(|x| {
                if has_name {
                    x.output == monitor_name
                } else {
                    true
                }
            }).collect();

            println!("{:#?}", test);
        }


        //ButtonBox ?

        let mut labels: Vec<Label> = Vec::new();

        gtk::timeout_add(100, move || {

            if let Ok(workspaces) = connection.get_workspaces() {
                let filtered: Vec<_> = workspaces.workspaces.iter().filter(|x| {
                    if has_name {
                        x.output == monitor_name
                    } else {
                        true
                    }
                }).collect();

                for (i, workspace) in filtered.iter().enumerate() {
                    let added_new = if let Some(label) = labels.get_mut(i) {
                        // if a label exists
                        label.set_label(&workspace.name);
                        if let Some(ctx) = label.get_style_context() {
                            if workspace.focused {
                                StyleContextExt::add_class(&ctx, "focused");
                            }
                            else {
                                StyleContextExt::remove_class(&ctx, "focused");
                            }
                        }
                        None
                    }
                    else {
                        // otherwise, create a new one
                        let label = Label::new(None);
                        label.set_label(&workspace.name);
                        wrapper.add(&label);
                        wrapper.show_all();
                        Some(label)
                    };
                    if let Some(added) = added_new {
                        // if we created a new one, add it to the vec
                        labels.push(added);
                    }
                }
                // remove extra items
                let work_len = filtered.len();
                let label_len = labels.len();
                if label_len > work_len {
                    labels.splice(work_len.., vec![]).for_each(|w| {
                        wrapper.remove(&w);
                    });
                }

            }
            gtk::Continue(true)
        });


    }
}
