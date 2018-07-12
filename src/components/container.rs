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

            // get fixed position
            // let fixed = config.get_vec_or("fixed", vec![]);
            // let fixed = if let Some(Property::Float(x)) = fixed.get(0) {
            //     if let Some(Property::Float(y)) = fixed.get(1) {
            //         Some((x, y))
            //     } else { None }
            // } else { None };

            // if let Some(fixed) {

            // } else {

            // }

            let wrapper = Box::new(direction, spacing);
            Self::init_widget(&wrapper, config);
            super::layout_to_container(&wrapper, layout, bar);
            container.add(&wrapper);
            wrapper.show();
        }
    }
}
