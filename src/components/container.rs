use super::{Component, Bar, ComponentConfig, Property};
use gtk::prelude::*;
use gtk::{Box, Orientation};

pub struct Container {
}

impl Component for Container {
    fn init(container: &Box, config: &ComponentConfig, bar: &Bar) {
        if let Some(layout) = config.properties.get("layout") {

            // get spacing
            let spacing = config.properties.get("spacing");
            let spacing = if let Some(&Property::Integer(ref sp)) = spacing {
                *sp
            } else {
                0
            }.abs() as i32;

            // get direction
            let direction = match config.properties.get("direction") {
                Some(&Property::String(ref dir)) => if dir.as_str() == "column" {
                    Orientation::Vertical
                } else {
                    Orientation::Horizontal
                }, _ => Orientation::Horizontal,
            };

            let new_container = Box::new(direction, spacing);
            WidgetExt::set_name(&new_container, &config.name);
            super::layout_to_container(&new_container, layout, bar);
            container.add(&new_container);
        }
    }
}
