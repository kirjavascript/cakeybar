// use systemstat::{System, Platform};

use super::{Component, Bar, gtk, ComponentConfig};
use gtk::prelude::*;
use gtk::{Label};

// use probes::{cpu, network, load};
//use sysinfo::{ProcessExt, SystemExt};
use sysinfo::{ProcessorExt, SystemExt, System};

pub struct CPU { }

impl Component for CPU {
    fn init(container: &gtk::Box, config: &ComponentConfig, _bar: &Bar) {
        let label = Label::new(None);
        CPU::init_widget(&label, config);
        container.add(&label);
        label.show();

        // let mut system = System::new();

        // let label_clone = label.clone();
        // let tick = move || {
        //     system.refresh_all();
        //     let processor_list = system.get_processor_list();
        //     if !processor_list.is_empty() {
        //         let pro = &processor_list[0];
        //         label_clone.set_text(format!("{:.2} %", pro.get_cpu_usage() * 100.).as_str());
        //     } else {
        //         label_clone.set_text("0.0 %");
        //     }
        //     // let text = format!("{}", processor.get_cpu_usage());
        //     // label_tick_clone.set_text(&text);
        //     gtk::Continue(true)
        // };

        let interval = config.get_int_or("interval", 1);
        tick();
        gtk::timeout_add_seconds(interval as u32, tick);
    }
}
