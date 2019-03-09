use crate::bar::Bar;
use crate::components::Component;
use crate::config::ConfigGroup;
use gtk;
use gtk::prelude::*;
use crate::util::{format_bytes, LabelGroup, SymbolFmt, Timer};

use probes::disk_usage;

pub struct Disk {
    wrapper: gtk::Box,
    timer: Timer,
}

impl Component for Disk {
    fn destroy(&self) {
        self.timer.remove();
        self.wrapper.destroy();
    }
}

impl Disk {
    pub fn init(config: ConfigGroup, bar: &mut Bar, container: &gtk::Box) {
        let label_group = LabelGroup::new();
        super::init_widget(&label_group.wrapper, &config, bar, container);

        let mounts = config.get_string_vec("mounts");
        let symbols = SymbolFmt::new(config.get_str_or("format", "{free}"));

        let should_include = move |s: &str| mounts.len() == 0 || mounts.contains(&&s.to_string());

        let tick = clone!(label_group move || {
            let labels = match disk_usage::read() {
                Ok(disks) => {
                    disks
                        .iter()
                        .fold(vec![], |mut acc, disk| {
                            if should_include(&disk.mountpoint) {
                                let text = symbols.format(|sym| {
                                    match sym {
                                        "free" => format_bytes(disk.one_k_blocks_free * 1024),
                                        "used" => format_bytes(disk.one_k_blocks_used * 1024),
                                        "total" => format_bytes(disk.one_k_blocks * 1024),
                                        "fs" => disk.filesystem.clone().unwrap_or_else(|| {
                                            "".to_string()
                                        }),
                                        "mount" => disk.mountpoint.to_owned(),
                                        _ => sym.to_string(),
                                    }
                                });
                                acc.push(text)
                            }
                            acc
                        })
                },
                Err(err) => {
                    vec![err.to_string()]
                },
            };

            label_group.set(&labels);

            gtk::Continue(true)
        });

        let interval = config.get_int_or("interval", 3).max(1);
        let timer = Timer::add_seconds(interval as u32, tick);

        bar.add_component(Box::new(Disk {
            wrapper: label_group.wrapper,
            timer,
        }));
    }
}
