extern crate i3ipc;

use super::{Component, Bar, gtk, ComponentConfig};
use gtk::prelude::*;
use gtk::{Label, Box, Orientation, LabelExt};
use gdk::{Screen, ScreenExt, Rectangle};

use self::i3ipc::I3Connection;


pub struct I3Workspace {
}

impl Component for I3Workspace {
    fn init(container: &gtk::Box, config: &ComponentConfig, _bar: &Bar){

        // get spacing
        let spacing = config.get_int_or("spacing", 0) as i32;

        let wrapper = Box::new(Orientation::Horizontal, spacing);
        WidgetExt::set_name(&wrapper, &config.name);
        I3Workspace::align_item(&wrapper, config);
        container.add(&wrapper);


        let mut connection = I3Connection::connect().unwrap();

        let workspaces = connection.get_workspaces();

        let mut labels: Vec<Label> = Vec::new();
        if let Ok(workspaces) = connection.get_workspaces() {


            for (i, workspace) in workspaces.workspaces.iter().enumerate() {
                // println!("{:#?}", workspace);
            // let children = wrapper.get_children();
                // let label = labels.get(i);
                // if let Some(label) = label {
                //     label.set_text(&"asdasd");
                // } else {
                    let label = Label::new(None);
                    label.set_text(&workspace.name);
                    wrapper.add(&label);
                    // labels.push(label);
                // };
            }

        }

        // let label_clone = label.clone();
        gtk::timeout_add(10, move || {
            // label_clone.set_text(&format!("{:?}", connection.get_workspaces()));

            if let Ok(workspaces) = connection.get_workspaces() {

            }
            gtk::Continue(true)
        });


    }
}
