use super::{Component, Bar, gtk, ComponentConfig};
use util::{format_bytes, format_symbols, LabelGroup};
use sysinfo::{DiskExt, SystemExt, System};

pub struct Disk { }

impl Component for Disk {
    fn init(container: &gtk::Box, config: &ComponentConfig, bar: &Bar) {
        let label_group = LabelGroup::new();
        Self::init_widget(&label_group.wrapper, container, config, bar);

        let mut system = System::new();

        let mounts = config.get_string_vec("mounts");

        let should_include = move |s: &str| {
            if mounts.len() == 0 || mounts.contains(&&s.to_string()) {
                true
            } else {
                false
            }
        };

        let format_str = config.get_str_or("format", "{free}").to_string();

        let mut tick = clone!((format_str, label_group) move || {
            system.refresh_disk_list();

            let labels = system.get_disks()
                .iter()
                .fold(vec![], |mut acc, disk| {
                    if let Some(mount_point) = disk.get_mount_point().to_str() {
                        if should_include(mount_point) {
                            let text = format_symbols(&format_str, |sym| {
                                match sym {
                                    "free" => format_bytes(disk.get_available_space()),
                                    "total" => format_bytes(disk.get_total_space()),
                                    "type" => format!("{:?}", disk.get_type()),
                                    "name" => disk.get_name().to_str().unwrap_or("?").to_string(),
                                    "path" => disk.get_mount_point().to_str().unwrap_or("?").to_string(),
                                    _ => sym.to_string(),
                                }
                            });
                            acc.push(text)
                        }
                    }
                    acc
                });

            label_group.set(labels);

            gtk::Continue(true)
        });

        let interval = config.get_int_or("interval", 3).max(1);
        tick();
        gtk::timeout_add_seconds(interval as u32, tick);
    }
}
