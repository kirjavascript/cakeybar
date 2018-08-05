use super::{Component, Bar, gtk, ComponentConfig};
use gtk::prelude::*;
use gtk::{Label};
use util::{format_bytes, format_symbols};
use config::Property;

use sysinfo::{DiskExt, SystemExt, System};

pub struct Disk { }


impl Component for Disk {
    fn init(container: &gtk::Box, config: &ComponentConfig, bar: &Bar) {
        let wrapper = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        Self::init_widget(&wrapper, container, config, bar);
        wrapper.show();
        let mut system = System::new();

        let mounts = config.get_vec_or("mounts", vec![])
            .iter()
            .fold(vec![], |mut acc, cur| {
                if let Property::String(s) = cur {
                    acc.push(s.clone());
                }
                acc
            });

        let should_include = move |s: &str| {
            if mounts.len() == 0 || mounts.contains(&&s.to_string()) {
                true
            } else {
                false
            }
        };

        let format_str = config.get_str_or("format", "{free}").to_string();

        let mut tick = clone!((wrapper, format_str) move || {
            system.refresh_disk_list();
            // remove old labels from wrapper
            for child in wrapper.get_children() {
                child.destroy();
            }

            for disk in system.get_disks() {
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
                        let label = Label::new(None);
                        label.set_text(&text);
                        label.show();
                        wrapper.add(&label);
                    }
                }
            }
            gtk::Continue(true)
        });

        let interval = config.get_int_or("interval", 3).max(1);
        tick();
        gtk::timeout_add_seconds(interval as u32, tick);
    }
}
