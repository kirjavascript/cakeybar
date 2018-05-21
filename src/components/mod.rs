// Component Template:
//
// use super::{Component, Bar, gtk, ComponentConfig};
// use gtk::prelude::*;
// use gtk::{Label};
//
// pub struct Template { }
//
// impl Component for Template {
//     fn init(container: &gtk::Box, config: &ComponentConfig, _bar: &Bar) {
//         let label = Label::new(None);
//         Template::init_widget(&label, &config.name);
//         label.set_text(&"test");
//         container.add(&label);
//     }
// }
extern crate glib;

use super::gtk;
use super::gtk::{Box, Align, WidgetExt};
use super::bar::Bar;
use super::config::{ComponentConfig, Property};

mod clock;
mod container;
mod i3window;
mod i3workspace;
mod image;
mod network;
mod void;

pub trait Component {
    fn init(container: &Box, config: &ComponentConfig, bar: &Bar);
    fn init_widget<T>(widget: &T, config: &ComponentConfig)
        where T: gtk::IsA<gtk::Widget>
            + gtk::IsA<gtk::Object>
            + glib::value::SetValue {
        // set name
        widget.set_name(&config.name);
        // align
        let halign_str = config.get_str_or("halign", "null");
        if halign_str != "null" {
            WidgetExt::set_halign(widget, Self::get_alignment(halign_str));
            WidgetExt::set_hexpand(widget, true);
        }
        let valign_str = config.get_str_or("valign", "null");
        if valign_str != "null" {
            WidgetExt::set_valign(widget, Self::get_alignment(valign_str));
            WidgetExt::set_vexpand(widget, true);
        }
    }
    fn get_alignment(align: &str) -> Align {
        match align {
            "start" => Align::Start,
            "end" => Align::End,
            "center" | "centre" => Align::Center,
            "fill" => Align::Fill,
            _ => Align::Baseline,
        }
    }
}

pub fn load_components(container: &Box, bar: &Bar) {
    layout_to_container(container, &bar.config.layout, bar);
}

fn layout_to_container(container: &Box, layout: &Property, bar: &Bar) {
    if let &Property::Array(ref arr) = layout {
        // iterate over layout
        arr.iter().for_each(|name_prop| {
            if let &Property::String(ref name) = name_prop {
                // get config for layout fragment
                let component_config = bar.components.iter().find(|x| {
                    &x.name == name
                });
                if let Some(config) = component_config {
                    load_component(container, config, bar);
                }
            }
        });
    }
}

fn load_component(container: &Box, config: &ComponentConfig, bar: &Bar) {
    // get type
    let component_type_option = config.properties.get("type");
    if let Some(&Property::String(ref component_type)) = component_type_option {
        // decide which component to load
        let component_init = match component_type.as_str() {
            "clock" => clock::Clock::init,
            "container" => container::Container::init,
            "i3window" => i3window::I3Window::init,
            "i3workspace" => i3workspace::I3Workspace::init,
            "image" => image::Image::init,
            "network" => network::Network::init,
            _ => void::Void::init,
        };
        // load component
        component_init(container, config, bar);
    }
}
