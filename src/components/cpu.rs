use systemstat::{System, Platform};
use systemstat::data::IpAddr;

use super::{Component, Bar, gtk, ComponentConfig};
use gtk::prelude::*;
use gtk::{Label};

pub struct CPU { }

impl Component for CPU {
    fn init(container: &gtk::Box, config: &ComponentConfig, _bar: &Bar) {
        let label = Label::new(None);
        CPU::init_widget(&label, config);
        container.add(&label);

        let sys = System::new();

        let label_tick_clone = label.clone();
        let tick = move || {
            if let Ok(loadavg) = sys.load_average() {
                // label_tick_clone.set_text(&format!("1/{} 5/{} 15/{}", loadavg.one, loadavg.five, loadavg.fifteen));
            }
            gtk::Continue(true)
        };

        let interval = config.get_int_or("interval", 5);
        tick();
        gtk::timeout_add_seconds(interval as u32, tick);
    }
}
