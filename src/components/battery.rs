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
    fn init(container: &gtk::Box, config: &ComponentConfig, bar: &Bar) {
        let label = Label::new(None);
        Self::init_widget(&label, container, config, bar);
        label.show();

        let adapter = config.get_str_or("adapter", "AC").to_string();
        let battery = config.get_str_or("battery", "BAT0").to_string();
        let charge_full = get_value(battery.clone(), "charge_full").unwrap_or("0".to_string()).parse::<u64>();

        let tick = clone!(label move || {
            let plugged = get_value(adapter.clone(), "online").unwrap_or("0".to_string()) == "1".to_string();


            // display percentage
            if let Ok(full) = charge_full {
                let charge_now = get_value(battery.clone(), "charge_now").unwrap_or("0".to_string()).parse::<u64>();
                if let Ok(now) = charge_now {
                    // calculate pct
                    let pct = now as f64 / full as f64 * 100.;
                    let pct = pct.min(100.) as u8;
                    let is_full = pct >= 100;
                    let suffix = if is_full { "" } else { "%" };
                    label.set_text(&format!("{}{}", pct, suffix));

                    // decide on class
                    let class = match pct {
                        p if p >= 100 => "full",
                        p if p >= 65 => "high",
                        p if p >= 30 => "medium",
                        _ => "low",
                    };

                    // set classes
                    if let Some(ctx) = label.get_style_context() {
                        for class in ctx.list_classes().iter() {
                            ctx.remove_class(&class);
                        }
                        if plugged {
                            ctx.add_class("plugged");
                        }
                        ctx.add_class(class);
                    }
                }
            }

            gtk::Continue(true)
        });

        let interval = config.get_int_or("interval", 1);
        tick();
        gtk::timeout_add_seconds(interval as u32, tick);
    }
}
