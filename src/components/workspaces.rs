use super::{Component, Bar, gtk, ComponentConfig};
use gtk::prelude::*;
use gtk::{Label, Box, EventBox, Orientation, LabelExt, WidgetExt, StyleContextExt};

use wm;
use wm::events::{Event, EventValue};
use wm::workspace::Workspace;

use std::cell::RefCell;
use std::rc::Rc;

pub struct Workspaces { }

// Workspaces

impl Component for Workspaces {
    fn init(container: &Box, config: &ComponentConfig, bar: &Bar){
        let monitor_index = bar.config.get_int_or("monitor", 0) as i32;

        // get spacing
        let spacing = config.get_int_or("spacing", 0) as i32;

        // misc config
        let show_all = config.get_bool_or("show_all", false);
        let show_name = config.get_bool_or("show_name", false);

        // attach wrapper
        let wrapper = Box::new(Orientation::Horizontal, spacing);

        // add to container and show
        Self::init_widget(&wrapper, container, config, bar);
        wrapper.show();

        let name_opt = wm::gtk::get_monitor_name(monitor_index);

        let workspaces = bar.wm_util.get_workspaces().unwrap_or(vec![]);
        let workspaces = filter_by_name(&workspaces, show_all, &name_opt);

        // create initial UI

        let labels: Rc<RefCell<Vec<Label>>> = Rc::new(RefCell::new(
             Vec::new()
        ));

        let &Bar { wm_util, .. } = bar;

        for workspace in workspaces.iter() {
            let label = Label::new(None);
            set_label_attrs(&label, &workspace, show_name);
            let ebox = add_event_box(
                &label,
                workspace.name.clone(),
                wm_util.clone(),
            );
            wrapper.add(&ebox);
            labels.borrow_mut().push(label);
        }
        wrapper.show_all();

        // listen for events
        wm_util.add_listener(Event::Workspace, clone!((wrapper, labels, wm_util)
            move |workspaces_opt| {
                if let Some(EventValue::Workspaces(workspaces)) = workspaces_opt {

                    let mut workspaces = filter_by_name(&workspaces, show_all, &name_opt);

                    for (i, workspace) in workspaces.iter().enumerate() {
                        let added_new = if let Some(label) = labels.borrow_mut().get_mut(i) {
                            // if a label already exists
                            set_label_attrs(&label, &workspace, show_name);
                            None
                        } else {
                            // if adding a new label
                            let label = Label::new(None);
                            set_label_attrs(&label, &workspace, show_name);
                            let ebox = add_event_box(
                                &label,
                                workspace.name.clone(),
                                wm_util.clone(),
                            );
                            wrapper.add(&ebox);
                            Some(label)
                        };
                        if let Some(added) = added_new {
                            labels.borrow_mut().push(added);
                        }
                    }
                    wrapper.show_all();

                    // remove items
                    let work_len = workspaces.len();
                    let label_len = labels.borrow().len();
                    if label_len > work_len {
                        let mut labels = labels.borrow_mut();
                        labels.splice(work_len.., vec![]).for_each(|w| {
                            if let Some(parent) = w.get_parent() {
                                // nuke the event box
                                parent.destroy();
                            }
                        });
                    }
                }
            }
        ));

    }
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
        label.set_label(&workspace.number.to_string());
    };
    // style
    if let Some(ctx) = label.get_style_context() {
        let set_class = get_set_class(ctx);
        set_class("focused", workspace.focused);
        set_class("visible", workspace.visible);
        set_class("urgent", workspace.urgent);
    }
}

fn add_event_box(label: &Label, name: String, wm_util: wm::WMUtil) -> EventBox {
    let ebox = EventBox::new();
    ebox.add(label);
    ebox.connect_button_press_event(move |_, _| {
        wm_util.focus_workspace(&name);
        Inhibit(false)
    });
    ebox
}

fn filter_by_name<'a>(workspaces: &'a Vec<Workspace>, show_all: bool, name_opt: &Option<String>) -> Vec<&'a Workspace> {
    workspaces
        .iter()
        .filter(|w| {
            if show_all {
                true
            } else {
                match name_opt {
                    Some(ref name) => *name == w.output,
                    None => true,
                }
            }
        })
    .collect::<Vec<&Workspace>>()
}
