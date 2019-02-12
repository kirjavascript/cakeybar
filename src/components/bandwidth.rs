use crate::bar::Bar;
use crate::components::Component;
use crate::config::ConfigGroup;
use gtk;
use gtk::prelude::*;
use crate::util::{format_bytes, LabelGroup, SymbolFmt, Timer};

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use probes::network;

pub struct Bandwidth {
    config: ConfigGroup,
    wrapper: gtk::Box,
    timer: Timer,
}

impl Component for Bandwidth {
    fn get_config(&self) -> &ConfigGroup {
        &self.config
    }
    fn show(&self) {
        self.wrapper.show();
    }
    fn hide(&self) {
        self.wrapper.hide();
    }
    fn destroy(&self) {
        self.timer.remove();
        self.wrapper.destroy();
    }
}

impl Bandwidth {
    pub fn init(config: ConfigGroup, bar: &mut Bar, container: &gtk::Box) {

        let label_group = LabelGroup::new();
        super::init_widget(&label_group.wrapper, &config, bar, container);

        let interfaces = config.get_string_vec("interfaces");
        let interval = config.get_int_or("interval", 3).max(1) as u64;
        let symbols = SymbolFmt::new(config.get_str_or("format", "{down/s}"));

        let should_include =
            move |s: &str| interfaces.len() == 0 || interfaces.contains(&&s.to_string());

        // last frame of data
        let last: Rc<RefCell<HashMap<String, (u64, u64)>>> = Rc::new(RefCell::new(HashMap::new()));

        let name = config.name.clone();
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
                            let text = symbols.format(|sym| {
                                match sym {
                                    "name" => name.to_string(),
                                    "down/s" => {
                                        format!("{}/s", format_bytes(if rx > 0 {
                                            rx_now.max(rx) - rx
                                        } else {
                                            0
                                        } / interval))
                                    },
                                    "up/s" => {
                                        format!("{}/s", format_bytes(if tx > 0 {
                                            tx_now.max(tx) - tx
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
                    error!("#{}: {}", name, err);
                },
            }
            gtk::Continue(true)
        });
        let timer = Timer::add_seconds(interval as u32, tick);

        bar.add_component(Box::new(Bandwidth {
            config,
            wrapper: label_group.wrapper,
            timer,
        }));
    }
}
