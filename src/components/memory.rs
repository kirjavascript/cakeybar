use bar::Bar;
use components::Component;
use config::ConfigGroup;
use gtk;
use gtk::prelude::*;
use gtk::Label;
use util::{format_bytes, SymbolFmt, Timer};

use probes::memory;

pub struct Memory {
    config: ConfigGroup,
    label: Label,
    timer: Timer,
}

impl Component for Memory {
    fn get_config(&self) -> &ConfigGroup {
        &self.config
    }
    fn show(&mut self) {
        self.label.show();
    }
    fn hide(&mut self) {
        self.label.hide();
    }
    fn destroy(&self) {
        self.timer.remove();
        self.label.destroy();
    }
}

impl Memory {
    pub fn init(config: ConfigGroup, bar: &mut Bar, container: &gtk::Box) {
        let label = Label::new(None);
        super::init_widget(&label, &config, bar, container);
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

        bar.add_component(Box::new(Memory {
            config,
            label,
            timer,
        }));
    }
}
