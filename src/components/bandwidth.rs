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
        Bandwidth::init_widget(&label, config);
        container.add(&label);
        label.show();

        let interface = String::from(config.get_str_or("interface", "lo"));
        let interval = config.get_int_or("interval", 1) as u64;

        let mut recv = 0u64;
        let label_tick_clone = label.clone();
        let mut tick = move || {
            let bw = network::read();
            match bw {
                Ok(info) => {
                    let interface_opt = info.interfaces.iter().find(|x| x.0 == &interface);
                    if let Some((_name, interface)) = interface_opt {
                        let diff = interface.received - recv;
                        if recv != 0 {
                            label_tick_clone.set_text(
                                &bytes_to_string(diff / interval)
                            );
                        }
                        recv = interface.received;
                    }
                },
                Err(err) => {
                    eprintln!("{}", err);
                },
            }

            // label_tick_clone.set_text(&"Bandwidth: ?");
            gtk::Continue(true)
        };

        tick();
        gtk::timeout_add_seconds(interval as u32, tick);
    }
}
