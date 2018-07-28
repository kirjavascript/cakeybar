use super::{Component, Bar, gtk, ComponentConfig};
use gtk::prelude::*;
use gtk::{Label};
use util::format_bytes;

use sysinfo::{DiskExt, SystemExt, System};

pub struct Disk { }

impl Component for Disk {
    fn init(container: &gtk::Box, config: &ComponentConfig, bar: &Bar) {
        let wrapper = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        Self::init_widget(&wrapper, container, config, bar);
        wrapper.show();
        let mut system = System::new();

        let mut tick = clone!(wrapper move || {
            system.refresh_disk_list();
            // remove old labels from wrapper
            for child in wrapper.get_children() {
                child.destroy();
            }

            for disk in system.get_disks() {
                // let text = format!(
                //     "{:?} {:?} {:?} {}/{}",
                //     disk.get_type(),
                //     disk.get_name(),
                //     disk.get_mount_point(),
                //     format_bytes(disk.get_available_space()),
                //     format_bytes(disk.get_total_space()),
                // );
                let text = format!("{}", format_bytes(disk.get_available_space()));
                let label = Label::new(None);
                label.set_text(&text);
                label.show();
                wrapper.add(&label);
            }
            gtk::Continue(true)
        });

        let interval = config.get_int_or("interval", 5).max(1);
        tick();
        gtk::timeout_add_seconds(interval as u32, tick);
    }
}
