use super::{Component, Bar, gtk, ComponentConfig};
use gtk::prelude::*;
use gtk::{Label};
use util::format_bytes;

use probes::{network};

pub struct Bandwidth { }

impl Component for Bandwidth {
    fn init(container: &gtk::Box, config: &ComponentConfig, bar: &Bar) {
        let label = Label::new(None);
        Self::init_widget(&label, container, config, bar);
        label.show();

        let interface = String::from(config.get_str_or("interface", "auto"));
        let interval = config.get_int_or("interval", 3).max(1) as u64;

        let mut recv = 0u64;
        let mut tick = clone!(label move || {
            let bw = network::read();
            match bw {
                Ok(info) => {
                    let mut interface_opt = if interface == "auto" {
                        info.interfaces.iter().find(|_| true)
                    } else {
                        info.interfaces.iter().find(|x| x.0 == &interface)
                    };
                    if let Some((_name, interface)) = interface_opt {
                        let diff = if interface.received >= recv {
                            interface.received - recv
                        } else {
                            0
                        };
                        if recv != 0 {
                            label.set_text(
                                &format!("{}/s", format_bytes(diff / interval))
                            );
                        } else {
                            label.set_text(&"0B/s");
                        }
                        recv = interface.received;
                    }
                },
                Err(err) => {
                    error!("bandwidth: {}", err);
                },
            }
            gtk::Continue(true)
        });

        tick();
        gtk::timeout_add_seconds(interval as u32, tick);
    }
}
