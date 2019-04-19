use crate::components::{Component, ComponentParams};
use gtk::prelude::*;
use gtk::{Label, StyleContextExt};
use std::io::Error;
use crate::util::{read_file, SymbolFmt, Timer};

pub struct Battery {
    label: Label,
    timer: Timer,
}

impl Component for Battery {
    fn destroy(&self) {
        self.timer.remove();
        self.label.destroy();
    }
}

impl Battery {
    pub fn init(params: ComponentParams) {
        let ComponentParams { config, window, wm_util, container } = params;
        let label = Label::new(None);
        super::init_widget(&label, &config, &window, container);
        label.show();

        let adapter = config.get_str_or("adapter", "AC").to_string();
        let battery = config.get_str_or("battery", "BAT0").to_string();
        let has_battery = get_data(&battery, "charge_full").is_ok();

        let symbols = SymbolFmt::new(config.get_str_or("format", "{percent}"));

        if has_battery {
            let tick = clone!(label move || {
                if let Ok((full, now, current)) = get_charge(&battery) {
                    let plugged = get_data(&adapter, "online")
                        .unwrap_or("0".to_string()) == "1".to_string();

                    // calculate pct
                    let pct = now as f64 / full as f64 * 100.;
                    let pct = pct.min(100.) as u8;

                    // TODO: this seems incorrect
                    let remaining = now / current;

                    // set label
                    label.set_markup(&symbols.format(|sym| match sym {
                        "percent" => format!("{}%", pct),
                        "remaining" => format!(
                            "{}:{:0>2}?", remaining / 3600,
                            (remaining / 60) % 60,
                        ),
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
            let timer = Timer::add_seconds(interval as u32, tick);

            window.add_component(Box::new(Battery {
                label,
                timer,
            }));
        } else {
            warn!("no battery detected");
        }
    }
}

fn get_data(device: &str, query: &str) -> Result<String, Error> {
    read_file(&format!("/sys/class/power_supply/{}/{}", device, query))
}

fn get_charge(battery: &str) -> Result<(i32, i32, i32), Error> {
    let full = get_data(battery, "charge_full").map(|a| a.parse().unwrap_or(0));
    let now = get_data(battery, "charge_now").map(|a| a.parse().unwrap_or(0));
    let current = get_data(battery, "current_now").map(|a| a.parse().unwrap_or(0));
    full.and_then(|a| now.and_then(|b| current.and_then(|c| Ok((a, b, c)))))
}
