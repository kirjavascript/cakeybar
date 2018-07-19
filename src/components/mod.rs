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
//         Self:init_widget(&label, container, config, bar);
//     }
// }

use {gtk, glib};
use gtk::{Box, Align, WidgetExt, StyleContextExt, ContainerExt, OverlayExt};
use bar::Bar;
use config::{ComponentConfig, Property};

mod bandwidth;
mod battery;
mod clock;
mod container;
mod cpu;
mod dropdown;
mod image;
mod ip;
mod menu;
mod mode;
mod tray;
mod void;
mod window;
mod workspaces;


pub trait Component {
    fn init(container: &Box, config: &ComponentConfig, bar: &Bar);

    fn init_widget<T>(
        widget: &T,
        container: &Box,
        config: &ComponentConfig,
        bar: &Bar,
    ) where T: gtk::IsA<gtk::Widget>
            + gtk::IsA<gtk::Object>
            + glib::value::SetValue {
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

        // position = "fixed"
        // let fixed = config.get_vec_or("fixed", vec![]);
        // let fixed = if let Some(Property::Float(x)) = fixed.get(0) {
        //     if let Some(Property::Float(y)) = fixed.get(1) {
        //         Some((x, y))
        //     } else { None }
        // } else { None };
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

fn load_component(container: &Box, config: &ComponentConfig, bar: &Bar) {
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
            "dropdown" => dropdown::Dropdown::init,
            "image" => image::Image::init,
            "ip" => ip::IP::init,
            "menu" => menu::Menu::init,
            "mode" => mode::Mode::init,
            "tray" => tray::Tray::init,
            "window" => window::Window::init,
            "workspaces" => workspaces::Workspaces::init,
            _ => void::Void::init,
        };
        // load component
        component_init(container, config, bar);
    }
}

fn layout_to_container(container: &Box, layout: &Property, bar: &Bar) {
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

pub fn load_components(container: &Box, bar: &Bar) {
    let layout = Property::Array(bar.config.get_vec_or("layout", vec![]));
    layout_to_container(container, &layout, bar);
}
