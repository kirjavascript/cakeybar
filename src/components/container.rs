use crate::components::{Component, ComponentParams};
use gtk::prelude::*;
use gtk::Orientation;

pub struct Container {
    wrapper: gtk::Box,
}

impl Component for Container {
    fn destroy(&self) {
        self.wrapper.destroy();
    }
}

impl Container {
    pub fn init(params: ComponentParams) {
        let ComponentParams { config, window, wm_util, container } = params;
        // get spacing
        let spacing = config.get_int_or("spacing", 0) as i32;

        // get direction
        let direction = match config.get_str_or("direction", "horizontal") {
            "horizontal" => Orientation::Horizontal,
            _ => Orientation::Vertical,
        };

        let wrapper = gtk::Box::new(direction, spacing);
        super::init_widget(&wrapper, &config, &window, container);
        wrapper.show();

        // load layout
        for name in config.get_string_vec("layout") {
            // let wrapper = wrapper.clone();
            let config_opt = wm_util.get_component_config(&name);
            if let Some(config) = config_opt {
                // super::load_component(&wm_util, config, &window, Some(&wrapper));
                // super::load_component(ComponentParams {
                //     container: &wrapper,
                //     config,
                //     window,
                //     wm_util,
                // });
            } else {
                warn!("missing component #{}", name);
            }
        }

        window.add_component(Box::new(Container { wrapper }));
    }
}
