use crate::components::{Component, ComponentParams};
use gtk::prelude::*;
use gtk::Label;
use crate::util::{format_bytes, SymbolFmt, Timer};

use probes::memory;

pub struct Memory {
    label: Label,
    timer: Timer,
}

impl Component for Memory {
    fn destroy(&self) {
        self.timer.remove();
        self.label.destroy();
    }
}

impl Memory {
    pub fn init(params: ComponentParams) {
        let ComponentParams { config, window, container, .. } = params;
        let label = Label::new(None);
        super::init_widget(&label, &config, &window, container);
        label.show();

        let symbols = SymbolFmt::new(config.get_str_or("format", "{free-pct}"));

        let name = config.name.clone();
        let tick = clone!(label move || {
            match memory::read() {
                Ok(data) => {
                    label.set_markup(&symbols.format(|sym| {
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
        let timer = Timer::add_seconds(interval as u32, tick);

        window.add_component(Box::new(Memory {
            label,
            timer,
        }));
    }
}
