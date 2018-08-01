// Component Template:
//
// use super::{Component, Bar, gtk, ComponentConfig};
// use gtk::prelude::*;
// use gtk::{Label};
//
// pub struct Template { }
//
// impl Component for Template {
//     fn init(container: &gtk::Box, config: &ComponentConfig, bar: &Bar) {
//         let label = Label::new(None);
//         label.set_text(&"test");
//         label.show();
//         Self::init_widget(&label, container, config, bar);
//     }
// }

use {gtk, glib};
use gtk::{
    Align,
    WidgetExt,
    StyleContextExt,
    ContainerExt,
    OverlayExt,
};
use bar::Bar;
use config::{ComponentConfig, Property};

mod bandwidth;
mod battery;
mod clock;
mod container;
mod cpu;
mod disk;
mod dropdown;
mod equalizer;
mod image;
mod ip;
mod menu;
mod mode;
mod script;
mod tray;
mod void;
mod window;
mod workspaces;


pub trait Component {
    fn init(container: &gtk::Box, config: &ComponentConfig, bar: &Bar);

    fn init_widget<T>(
        widget: &T,
        container: &gtk::Box,
        config: &ComponentConfig,
        bar: &Bar,
    ) where T: gtk::IsA<gtk::Widget>
            + gtk::IsA<gtk::Object>
            + glib::value::SetValue {
        // add wrapper
        // let wrapper = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        // wrapper.add(widget);
        // wrapper.show();
        // let widget = &wrapper;
        // set name
        widget.set_name(&config.name);
        // class
        let class_str = config.get_str_or("class", "null");
        if class_str != "null"  {
            if let Some(ctx) = widget.get_style_context() {
                ctx.add_class(class_str);
            }
        }
        let is_fixed = config.get_bool_or("fixed", false);
        // set alignment
        let halign_str = config.get_str_or("halign", "null");
        if halign_str != "null" {
            WidgetExt::set_halign(widget, Self::get_alignment(halign_str));
            if !is_fixed {
                WidgetExt::set_hexpand(widget, true);
            }
        }
        let valign_str = config.get_str_or("valign", "null");
        if valign_str != "null" {
            WidgetExt::set_valign(widget, Self::get_alignment(valign_str));
            if !is_fixed {
                WidgetExt::set_vexpand(widget, true);
            }
        }
        // set layout type
        if is_fixed {
            bar.overlay.add_overlay(widget);
            if config.get_bool_or("pass_through", true) {
                bar.overlay.set_overlay_pass_through(widget, true);
            }
        } else {
            container.add(widget);
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

fn load_component(container: &gtk::Box, config: &ComponentConfig, bar: &Bar) {
    // get type
    let component_type_option = config.properties.get("type");
    if let Some(&Property::String(ref component_type)) = component_type_option {
        // decide which component to load
        let component_init = match component_type.as_str() {
            "bandwidth" => bandwidth::Bandwidth::init,
            "battery" => battery::Battery::init,
            "clock" => clock::Clock::init,
            "container" => container::Container::init,
            "cpu" => cpu::CPU::init,
            "disk" => disk::Disk::init,
            "dropdown" => dropdown::Dropdown::init,
            "equalizer" | "equaliser" => equalizer::Equalizer::init,
            "image" => image::Image::init,
            "ip" => ip::IP::init,
            "menu" => menu::Menu::init,
            "mode" => mode::Mode::init,
            "script" => script::Script::init,
            "tray" => tray::Tray::init,
            "window" => window::Window::init,
            "workspaces" => workspaces::Workspaces::init,
            _ => void::Void::init,
        };
        // load component
        component_init(container, config, bar);
    }
}

fn layout_to_container(container: &gtk::Box, layout: &Property, bar: &Bar) {
    if let &Property::Array(ref arr) = layout {
        // iterate over layout
        arr.iter().for_each(|name_prop| {
            if let &Property::String(ref name) = name_prop {
                // get config for layout fragment
                let component_config = bar.app_config.components.iter().find(|x| {
                    &x.name == name
                });
                if let Some(config) = component_config {
                    load_component(container, config, bar);
                }
            }
        });
    }
}

pub fn load_components(container: &gtk::Box, bar: &Bar) {
    let layout = Property::Array(bar.config.get_vec_or("layout", vec![]));
    layout_to_container(container, &layout, bar);
}
