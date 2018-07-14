use super::{Component, Bar, gtk, ComponentConfig};
use gtk::prelude::*;
use gtk::{Label};
use chrono::Local;

pub struct Script { }

impl Component for Script {
    fn init(container: &gtk::Box, config: &ComponentConfig, _bar: &Bar) {
        let label = Label::new(None);
        Self::init_widget(&label, config);

        let format = config.get_str_or("format", "%Y-%m-%d %H:%M:%S").to_string();

        label.set_text(&current_time(format.clone()));

        let tick = clone!(label move || {
            label.set_text(&current_time(format.clone()));
            gtk::Continue(true)
        });

        let interval = config.get_int_or("interval", 5);
        gtk::timeout_add_seconds(interval as u32, tick);

        container.add(&label);
        label.show();

    }
}
