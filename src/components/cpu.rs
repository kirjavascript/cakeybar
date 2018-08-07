use super::{Component, Bar, gtk, ComponentConfig};
use gtk::prelude::*;
use gtk::{Label};

use sysinfo::{ProcessorExt, SystemExt, System};

pub struct CPU { }

// TODO: system_stat cpu temp
// https://github.com/myfreeweb/systemstat/blob/master/src/platform/linux.rs#L505
// util::read_file Battery

impl Component for CPU {
    fn init(container: &gtk::Box, config: &ComponentConfig, bar: &Bar) {
        let label = Label::new(None);
        Self::init_widget(&label, container, config, bar);
        label.show();

        let mut system = System::new();

        let mut tick = clone!(label move || {
            system.refresh_system();
            let processor_list = system.get_processor_list();
            if !processor_list.is_empty() {
                let pro = &processor_list[0];
                label.set_text(format!("{:.2}%", pro.get_cpu_usage() * 100.).as_str());
            } else {
                label.set_text("0.00%");
            }
            gtk::Continue(true)
        });

        let interval = config.get_int_or("interval", 3).max(1);
        tick();
        gtk::timeout_add_seconds(interval as u32, tick);
    }
}
