use super::{Component, Bar, gtk, ConfigGroup};
use gtk::prelude::*;
use gtk::{Label};
use util::{SymbolFmt, read_file};

use sysinfo::{ProcessorExt, SystemExt, System};

pub struct CPU;

impl Component for CPU {
    fn init(container: &gtk::Box, config: &ConfigGroup, bar: &Bar) {
        let label = Label::new(None);
        Self::init_widget(&label, container, config, bar);
        label.show();

        let mut system = System::new();
        let symbols = SymbolFmt::new(config.get_str_or("format", "{usage}"));
        let has_usage = symbols.contains("usage");

        let mut tick = clone!(label move || {
            if has_usage {
                system.refresh_system();
            }
            label.set_markup(&symbols.format(|sym| match sym {
                "usage" => {
                    let processor_list = system.get_processor_list();
                    if !processor_list.is_empty() {
                        let pro = &processor_list[0];
                        format!("{:.2}%", pro.get_cpu_usage() * 100.)
                    } else {
                        "NOCPU".to_string()
                    }
                },
                "temp" | "dumbtemp" => {
                    match read_file("/sys/class/thermal/thermal_zone0/temp") {
                        Ok(text) => match text.parse::<f32>() {
                            Ok(num) => {
                                if sym == "temp" {
                                    format!("{}°C", num / 1000.)
                                } else {
                                    format!("{:.0}°F", ((num / 1000.) * 1.8) + 32.)
                                }
                            },
                            Err(_) => "NOTEMP".to_string(),
                        },
                        Err(_) => "NOTEMP".to_string(),
                    }
                },
                _ => sym.to_string(),
            }));
            gtk::Continue(true)
        });

        let interval = config.get_int_or("interval", 3).max(1);
        tick();
        gtk::timeout_add_seconds(interval as u32, tick);
    }
}
