use super::gtk;
use super::bar::Bar;
use super::config::{ComponentConfig, Property};

mod clock;
mod container;
mod image;
mod void;

pub trait Component {
    fn init(container: &gtk::Box, config: &ComponentConfig, bar: &Bar);
}

pub fn load_components(container: &gtk::Box, bar: &Bar) {
    layout_to_container(container, &bar.config.layout, bar);
}

fn layout_to_container(container: &gtk::Box, layout: &Property, bar: &Bar) {
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

fn load_component(container: &gtk::Box, config: &ComponentConfig, bar: &Bar) {
    // get type
    let component_type_option = config.properties.get("type");
    if let Some(&Property::String(ref component_type)) = component_type_option {
        // decide which component to load
        let component_init = match component_type.as_str() {
            "clock" => clock::Clock::init,
            "container" => container::Container::init,
            "image" => image::Image::init,
            _ => void::Void::init,
        };
        // load component
        component_init(container, config, bar);
    }
}
