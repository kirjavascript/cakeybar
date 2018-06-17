use super::{Component, Bar, gtk, ComponentConfig};
use gtk::prelude::*;
use gtk::{Label, StyleContextExt};

use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::path::Path;

pub struct Battery { }

fn get_value(device: String, query: &str) -> Result<String, Error> {
    let path = format!("/sys/class/power_supply/{}/{}", device, query);
    let path = Path::new(&path);
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.trim().to_string())
}

impl Component for Battery {
    fn init(container: &gtk::Box, config: &ComponentConfig, _bar: &Bar) {
        let label = Label::new(None);
        Self::init_widget(&label, &config);
        container.add(&label);
        label.show();

        let adapter = config.get_str_or("adapter", "AC").to_string();
        let battery = config.get_str_or("battery", "BAT0").to_string();
        let charge_full = get_value(battery.clone(), "charge_full").unwrap_or("0".to_string()).parse::<u64>();

        // set class for charging

        let tick = enclose!(label move || {
            let plugged = get_value(adapter.clone(), "online").unwrap_or("0".to_string()) == "1".to_string();

            if let Some(ctx) = label.get_style_context() {
                if plugged {
                    StyleContextExt::add_class(&ctx, "charging");
                } else {
                    StyleContextExt::remove_class(&ctx, "charging");
                }
            }

            // display percentage
            if let Ok(full) = charge_full {
                let charge_now = get_value(battery.clone(), "charge_now").unwrap_or("0".to_string()).parse::<u64>();
                if let Ok(now) = charge_now {
                    let pct = now as f64 / full as f64 * 100.;
                    let pct = pct.min(100.) as u8;
                    label.set_text(&format!("{}%", pct));
                }
            }

            gtk::Continue(true)
        });

        let interval = config.get_int_or("interval", 1);
        tick();
        gtk::timeout_add_seconds(interval as u32, tick);
    }
}
