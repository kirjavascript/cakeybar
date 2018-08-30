use super::{Component, Bar, gtk, ConfigGroup};
use gtk::prelude::*;
use gtk::{Label};
use chrono::Local;
use util::SymbolFmt;

pub struct Clock;

fn current_time(format: String) -> String {
    return format!("{}", Local::now().format(&format));
}

impl Component for Clock {
    fn init(container: &gtk::Box, config: &ConfigGroup, bar: &Bar) {
        let label = Label::new(None);

        let timestamp = config.get_str_or("timestamp", "%Y-%m-%d %H:%M:%S").to_string();
        let symbols = SymbolFmt::new(config.get_str_or("format", "{timestamp}"));

        let tick = clone!(label move || {
            let time = &current_time(timestamp.clone());
            label.set_markup(&symbols.format(|sym| {
                match sym {
                    "timestamp" => time.to_string(),
                    _ => sym.to_string(),
                }
            }));
            gtk::Continue(true)
        });

        let interval = config.get_int_or("interval", 1).max(1);
        tick();
        gtk::timeout_add_seconds(interval as u32, tick);

        label.show();

        Self::init_widget(&label, container, config, bar);
    }
}
