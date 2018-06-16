use super::{Component, Bar, gtk, ComponentConfig};
use gtk::prelude::*;
use gtk::{Label};

use probes::{network};

pub struct Bandwidth { }

fn bytes_to_string(bytes: u64) -> String {
    const LEN: usize = 5;
    let bytes = bytes as f64;
    let sizes: [&str; LEN] = ["", "K", "M", "G", "T"];
    let index = ((bytes).ln() / 1024_f64.ln()).floor();
    let val = bytes / (1024_f64.powf(index));
    let index = index as usize;
    let suffix = if index < LEN { sizes[index] } else { "?" };
    format!("{:.*}{}B/s", if index < 2 { 0 } else { 2 }, val, suffix)
}

impl Component for Bandwidth {
    fn init(container: &gtk::Box, config: &ComponentConfig, _bar: &Bar) {
        let label = Label::new(None);
        Self::init_widget(&label, config);
        container.add(&label);
        label.show();

        let interface = String::from(config.get_str_or("interface", "auto"));
        let interval = config.get_int_or("interval", 5) as u64;

        let mut recv = 0u64;
        let mut tick = enclose!(label move || {
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
                                &bytes_to_string(diff / interval)
                            );
                        } else {
                            label.set_text(&"0B/s");
                        }
                        recv = interface.received;
                    }
                },
                Err(err) => {
                    eprintln!("{}", err);
                },
            }
            gtk::Continue(true)
        });

        tick();
        gtk::timeout_add_seconds(interval as u32, tick);
    }
}
