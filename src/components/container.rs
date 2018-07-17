use super::{Component, Bar, ComponentConfig};
use gtk::prelude::*;
use gtk::{Box, Orientation};

pub struct Container { }

impl Component for Container {
    fn init(container: &Box, config: &ComponentConfig, bar: &Bar) {
        if let Some(layout) = config.properties.get("layout") {

            // get spacing
            let spacing = config.get_int_or("spacing", 0) as i32;

            // get direction
            let direction = config.get_str_or("direction", "column");
            let direction = match direction {
                "column" | "horizontal" => Orientation::Horizontal,
                _ => Orientation::Vertical,
            };

            let wrapper = Box::new(direction, spacing);
            super::layout_to_container(&wrapper, layout, bar);
            Self::init_widget(&wrapper, container, config, bar);
            wrapper.show();
        }
    }
}
