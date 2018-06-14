use super::{Component, Bar, gtk, ComponentConfig};
use gtk::prelude::*;
use gtk::{Label};
use chrono::Local;

pub struct Clock { }

fn current_time(format: String) -> String {
    return format!("{}", Local::now().format(&format));
}

impl Component for Clock {
    fn init(container: &gtk::Box, config: &ComponentConfig, _bar: &Bar) {
        let label = Label::new(None);
        Self::init_widget(&label, config);

        let format = config.get_str_or("format", "%Y-%m-%d %H:%M:%S").to_string();

        label.set_text(&current_time(format.clone()));

        let tick = enclose!(label move || {
            label.set_text(&current_time(format.clone()));
            gtk::Continue(true)
        });

        let interval = config.get_int_or("interval", 1);
        gtk::timeout_add_seconds(interval as u32, tick);

        container.add(&label);
        label.show();

    }
}
