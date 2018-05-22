use systemstat::{System, Platform};
use systemstat::data::IpAddr;

use super::{Component, Bar, gtk, ComponentConfig};
use gtk::prelude::*;
use gtk::{Label};

pub struct Network { }

impl Component for Network {
    fn init(container: &gtk::Box, config: &ComponentConfig, _bar: &Bar) {
        let label = Label::new(None);
        Network::init_widget(&label, config);
        container.add(&label);
        label.show();

        let interface = String::from(config.get_str_or("interface", "null"));

        let sys = System::new();

        let label_tick_clone = label.clone();
        let tick = move || {
            if let Ok(interfaces) = sys.networks() {
                let find_interface = interfaces.iter().find(|x| x.0 == &interface);
                if let Some((_name, iface)) = find_interface {
                    if let IpAddr::V4(addr) = iface.addrs[0].addr {
                        label_tick_clone.set_text(&format!("{}", addr));
                    }
                }
            }
            gtk::Continue(true)
        };

        let interval = config.get_int_or("interval", 5);
        tick();
        gtk::timeout_add_seconds(interval as u32, tick);
    }
}
