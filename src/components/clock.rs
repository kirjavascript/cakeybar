extern crate chrono;

use super::{Component, Bar, gtk, ComponentConfig, Property};
use gtk::prelude::*;
use gtk::{Label};
use self::chrono::Local;

pub struct Clock {
}

fn current_time(format: String) -> String {
    return format!("{}", Local::now().format(&format));
}

impl Component for Clock {
    fn init(container: &gtk::Box, config: &ComponentConfig, _bar: &Bar) {
        let label = Label::new(None);
        WidgetExt::set_name(&label, &config.name);

        let format = config.properties.get("format");
        let format = if let Some(&Property::String(ref fmt)) = format {
            fmt
        } else {
            "%Y-%m-%d %H:%M:%S"
        }.to_string();

        label.set_text(&current_time(format.clone()));

        let label_tick_clone = label.clone();
        let tick = move || {
            label_tick_clone.set_text(&current_time(format.clone()));
            gtk::Continue(true)
        };

        gtk::timeout_add_seconds(1, tick);

        container.add(&label);
    }
}
