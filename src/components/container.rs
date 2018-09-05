// use super::{Component, Bar, ConfigGroup};
// use gtk::prelude::*;
// use gtk::{Box, Orientation};

// pub struct Container;

// impl Component for Container {
//     fn init(container: &Box, config: &ConfigGroup, bar: &Bar) {
//     }
// }

use gtk;
use gtk::prelude::*;
use gtk::Orientation;
use config::{ConfigGroup, Property};
use components::{Component};
use bar::Bar;
// use util::{SymbolFmt, Timer};

pub struct Container {
    config: ConfigGroup,
    wrapper: gtk::Box,
}

impl Component for Container {
    fn get_config(&self) -> &ConfigGroup {
        &self.config
    }
    fn show(&mut self) {
        self.wrapper.show();
    }
    fn hide(&mut self) {
        self.wrapper.hide();
    }
    fn destroy(&self) {
        self.wrapper.destroy();
    }
}

impl Container {
    pub fn init(
        config: ConfigGroup, bar: &mut Bar, container: &gtk::Box,
    ) {
        // get spacing
        let spacing = config.get_int_or("spacing", 0) as i32;

        // get direction
        let direction = match config.get_str_or("direction", "column") {
            "column" | "horizontal" => Orientation::Horizontal,
            _ => Orientation::Vertical,
        };

        let wrapper = gtk::Box::new(direction, spacing);

        // TODO: init widget
        container.add(&wrapper);
        wrapper.show();

        // load layout
        for name in config.get_string_vec("layout") {
            let config_opt = bar.wm_util.get_component_config(&name);
            if let Some(config) = config_opt {
                super::load_component(config, bar, &wrapper);
            } else {
                warn!("missing component {:?}", name);
            }
        }

        bar.add_component(Box::new(Container {
            config, wrapper,
        }));
    }
}
