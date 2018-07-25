use super::{Component, Bar, gtk, ComponentConfig};
use gtk::prelude::*;
use gtk::{Label};
use util;

pub struct Script { }

impl Component for Script {
    fn init(container: &gtk::Box, config: &ComponentConfig, bar: &Bar) {
        let label = Label::new(None);

        let exec = config.get_str_or("exec", "echo \"exec property missing\"");

        let tick = clone!(label move || {
            label.set_text(&"script-goes-here");
            gtk::Continue(true)
        });

        let interval = config.get_int_or("interval", 5);
        tick();
        gtk::timeout_add_seconds(interval as u32, tick);

        label.show();
        Self::init_widget(&label, container, config, bar);
    }
}
