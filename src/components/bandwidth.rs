use super::{Component, Bar, gtk, ComponentConfig};
use util::{format_bytes, format_symbols, LabelGroup};

use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

use probes::{network};

pub struct Bandwidth { }

impl Component for Bandwidth {
    fn init(container: &gtk::Box, config: &ComponentConfig, bar: &Bar) {
        let label_group = LabelGroup::new();
        Self::init_widget(&label_group.wrapper, container, config, bar);

        let interfaces = config.get_string_vec("interfaces");
        let interval = config.get_int_or("interval", 3).max(1) as u64;
        let format_str = config.get_str_or("format", "{down/s}").to_string();

        let should_include = move |s: &str| {
            interfaces.len() == 0 || interfaces.contains(&&s.to_string())
        };

        // last frame of data
        let last: Rc<RefCell<HashMap<String, (u64, u64)>>> = Rc::new(RefCell::new(
            HashMap::new()
        ));

        let tick = clone!((label_group, last) move || {
            let bw = network::read();
            match bw {
                Ok(info) => {
                    let mut labels = Vec::new();

                    for (name, interface) in info.interfaces.iter() {
                        if should_include(&name) {
                            // get last result
                            let (rx, tx) = *last.borrow()
                                .get(&name.to_string()).unwrap_or(&(0, 0));

                            // save a new backup
                            let (rx_now, tx_now) = (
                                interface.received,
                                interface.transmitted,
                            );

                            last.borrow_mut().insert(name.to_string(), (rx_now, tx_now));
                            let text = format_symbols(&format_str, |sym| {
                                match sym {
                                    "name" => name.to_string(),
                                    "down/s" => {
                                        format!("{}/s", format_bytes(if rx > 0 {
                                            rx_now - rx
                                        } else {
                                            0
                                        } / interval))
                                    },
                                    "up/s" => {
                                        format!("{}/s", format_bytes(if tx > 0 {
                                            tx_now - tx
                                        } else {
                                            0
                                        } / interval))
                                    },
                                    "down/total" => format_bytes(rx_now),
                                    "up/total" => format_bytes(tx_now),
                                    _ => sym.to_string(),
                                }
                            });

                            labels.push(text);
                        }
                    }

                    label_group.set(&labels);

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
