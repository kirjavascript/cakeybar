use super::{Component, Bar, gtk, ComponentConfig};
use gtk::prelude::*;
use gtk::{Label};
use util::{SymbolFmt, format_bytes};

use probes::memory;

pub struct Memory { }

impl Component for Memory {
    fn init(container: &gtk::Box, config: &ComponentConfig, bar: &Bar) {
        let label = Label::new(None);
        Self::init_widget(&label, container, config, bar);
        label.show();

        let symbols = SymbolFmt::new(config.get_str_or("format", "{free-pct}"));

        let name = config.name.clone();
        let tick = clone!(label move || {
            match memory::read() {
                Ok(data) => {
                    label.set_text(&symbols.format(|sym| {
                        match sym {
                            "total" => format_bytes(data.total() * 1024),
                            "free" => format_bytes(data.free() * 1024),
                            "free-pct" => format!(
                                "{:.2}%",
                                (data.free() as f64 / data.total() as f64) * 100.,
                            ),
                            "used" => format_bytes(data.used() * 1024),
                            "used-pct" => format!(
                                "{:.2}%",
                                (data.used() as f64 / data.total() as f64) * 100.,
                            ),
                            "swap-total" => format_bytes(data.swap_total() * 1024),
                            "swap-used" => format_bytes(data.swap_used() * 1024),
                            _ => sym.to_string(),
                        }
                    }));
                },
                Err(err) => {
                    error!("#{}: {}", name, err);
                },
            }
            gtk::Continue(true)
        });

        let interval = config.get_int_or("interval", 3).max(1);
        tick();
        gtk::timeout_add_seconds(interval as u32, tick);
    }
}
