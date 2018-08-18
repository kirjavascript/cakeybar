use super::{Component, Bar, gtk, ComponentConfig};
use gtk::prelude::*;
use gtk::{Label, StyleContextExt};
use util::{SymbolFmt, read_file};
use std::time::Duration;
use std::io::Error;

use systemstat::{System, Platform};

pub struct Battery;

fn get_data(device: &str, query: &str) -> Result<String, Error> {
    read_file(&format!("/sys/class/power_supply/{}/{}", device, query))
}

impl Component for Battery {
    fn init(container: &gtk::Box, config: &ComponentConfig, bar: &Bar) {

        // TODO: add duration, add symbols

        let label = Label::new(None);
        Self::init_widget(&label, container, config, bar);
        label.show();

        let adapter = config.get_str_or("adapter", "AC").to_string();
        let battery = config.get_str_or("battery", "BAT0").to_string();
        let has_battery = get_data(&battery, "charge_full").is_ok();

        let symbols = SymbolFmt::new(config.get_str_or("format", "{percent}"));
// {dplcsEnabled ? '✔' : '✗'}
//

        if has_battery {
            let tick = clone!(label move || {
                let full = get_data(&battery, "full");
                let now = get_data(&battery, "now");
                let current = get_data(&battery, "current_now");
                let data_all = full.map(|a| now.map(|b| current.map(|c| (a,b,c))));
                if let Ok(a) = data_all {
                    let plugged = get_data(&adapter, "online")
                        .unwrap_or("0".to_string()) == "1".to_string();

                    println!("{:#?}", a);
                    let q: () = a;

                }


                // let v: Result<Vec<T>, Error>
                    // let plugged = sys.on_ac_power().unwrap_or(true);
                    // let capacity = life.remaining_capacity;
                    // let time = life.remaining_time;
                    // let is_full = capacity > 1.;
                    // let pct = (capacity * 100.) as u8;

                    // label.set_text(&symbols.format(|sym| match sym {
                    //     "percent" => format!("{}%", pct),
                    //     _ => sym.to_string(),
                    // }));


                    // // decide on class
                    // let class = match pct {
                    //     p if p >= 99 => "full",
                    //     p if p >= 65 => "high",
                    //     p if p >= 30 => "medium",
                    //     _ => "low",
                    // };

                    // // set classes
                    // if let Some(ctx) = label.get_style_context() {
                    //     for class in ctx.list_classes().iter() {
                    //         ctx.remove_class(&class);
                    //     }
                    //     if plugged {
                    //         ctx.add_class("plugged");
                    //     }
                    //     ctx.add_class(class);
                    // }
                // }
                gtk::Continue(true)
            });

            let interval = config.get_int_or("interval", 3).max(1);
            tick();
            gtk::timeout_add_seconds(interval as u32, tick);
        } else {
            warn!("no battery detected");
        }

        // let tick = clone!(label move || {
        // let plugged = read_file(&get_path(adapter.clone(), "online"))
        //     .unwrap_or("0".to_string()) == "1".to_string();


        // // display percentage
        // if let Ok(full) = charge_full {
        //     let charge_now = read_file(&get_path(adapter.clone(), "charge_now"))
        //         .unwrap_or("0".to_string()).parse::<u64>();
        //     if let Ok(now) = charge_now {
        //         // calculate pct
        //         let pct = now as f64 / full as f64 * 100.;
        //         let pct = pct.min(100.) as u8;
        //         let is_full = pct >= 100;
        //         let suffix = if is_full { "" } else { "%" };
        //         label.set_text(&format!("{}{}", pct, suffix));

        //         // decide on class
        //         let class = match pct {
        //             p if p >= 100 => "full",
        //             p if p >= 65 => "high",
        //             p if p >= 30 => "medium",
        //             _ => "low",
        //         };

        //         // set classes
        //         if let Some(ctx) = label.get_style_context() {
        //             for class in ctx.list_classes().iter() {
        //                 ctx.remove_class(&class);
        //             }
        //             if plugged {
        //                 ctx.add_class("plugged");
        //             }
        //             ctx.add_class(class);
        //         }
        //     }
        // }

        // gtk::Continue(true)
        // });

    }
}
