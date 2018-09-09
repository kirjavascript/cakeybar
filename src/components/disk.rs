use bar::Bar;
use components::Component;
use config::ConfigGroup;
use gtk;
use gtk::prelude::*;
use sysinfo::{DiskExt, System, SystemExt};
use util::{format_bytes, LabelGroup, SymbolFmt, Timer};

pub struct Disk {
    config: ConfigGroup,
    wrapper: gtk::Box,
    timer: Timer,
}

impl Component for Disk {
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

impl Disk {
    pub fn init(config: ConfigGroup, bar: &mut Bar, container: &gtk::Box) {
        let label_group = LabelGroup::new();
        super::init_widget(&label_group.wrapper, &config, bar, container);

        let mut system = System::new();

        let mounts = config.get_string_vec("mounts");
        let symbols = SymbolFmt::new(config.get_str_or("format", "{free}"));

        let should_include = move |s: &str| mounts.len() == 0 || mounts.contains(&&s.to_string());

        let tick = clone!(label_group move || {
            system.refresh_disk_list();

            let labels = system.get_disks()
                .iter()
                .fold(vec![], |mut acc, disk| {
                    if let Some(mount_point) = disk.get_mount_point().to_str() {
                        if should_include(mount_point) {
                            let text = symbols.format(|sym| {
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

            label_group.set(&labels);

            gtk::Continue(true)
        });

        let interval = config.get_int_or("interval", 3).max(1);
        let timer = Timer::add_seconds(interval as u32, tick);

        bar.add_component(Box::new(Disk {
            config,
            wrapper: label_group.wrapper,
            timer,
        }));
    }
}
