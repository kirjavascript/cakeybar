use super::{Component, Bar, gtk, ComponentConfig};
use gtk::prelude::*;
use gtk::{Label, StyleContextExt};
use util::{SymbolFmt, read_file};
use std::time::Duration;
use std::io::Error;

use systemstat::{System, Platform};

pub struct Battery;

impl Component for Battery {
    fn init(container: &gtk::Box, config: &ComponentConfig, bar: &Bar) {

        // TODO: add duration

        let label = Label::new(None);
        Self::init_widget(&label, container, config, bar);
        label.show();

        let adapter = config.get_str_or("adapter", "AC").to_string();
        let battery = config.get_str_or("battery", "BAT0").to_string();
        let has_battery = get_data(&battery, "charge_full").is_ok();

        let symbols = SymbolFmt::new(config.get_str_or("format", "{percent}"));

        if has_battery {
        // if true {
            let tick = clone!(label move || {
                if let Ok((full, now, current)) = get_charge(&battery) {
                    let plugged = get_data(&adapter, "online")
                        .unwrap_or("0".to_string()) == "1".to_string();

                    // calculate pct
                    let pct = now as f64 / full as f64 * 100.;
                    let pct = pct.min(100.) as u8;
                    // let is_full = pct >= 100;

                    label.set_text(&symbols.format(|sym| match sym {
                        "percent" => format!("{}%", pct),
                        "plugged" => format!("{}", if plugged {'✔'} else {'✗'}),
                        _ => sym.to_string(),
                    }));

                    // decide on class
                    let class = match pct {
                        p if p >= 99 => "full",
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
                gtk::Continue(true)
            });

            let interval = config.get_int_or("interval", 3).max(1);
            tick();
            gtk::timeout_add_seconds(interval as u32, tick);
        } else {
            warn!("no battery detected");
        }

    }
}

fn get_data(device: &str, query: &str) -> Result<String, Error> {
    read_file(&format!("/sys/class/power_supply/{}/{}", device, query))
}

fn get_charge(battery: &str) -> Result<(u64, u64, u64), Error> {
    let full = get_data(battery, "full").map(|a| a.parse().unwrap_or(0));
    let now = get_data(battery, "now").map(|a| a.parse().unwrap_or(0));
    let current = get_data(battery, "current_now").map(|a| a.parse().unwrap_or(0));
    // let full: Result<u64, ()> = Ok("6663000".to_string()).map(|a| a.parse().unwrap_or(0));
    // let now: Result<u64, ()> = Ok("7368000".to_string()).map(|a| a.parse().unwrap_or(0));
    // let current: Result<u64, ()> = Ok("1000".to_string()).map(|a| a.parse().unwrap_or(0));
    full.and_then(|a| now.and_then(|b| current.and_then(|c| {
        Ok((a, b, c))
    })))
}
