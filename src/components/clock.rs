use gtk;
use gtk::prelude::*;
use gtk::Label;
use config::{ConfigGroup, Property};
use components::{Component};
use bar::Bar;
use chrono::Local;
use util::{SymbolFmt, Timer};

pub struct Clock {
    config: ConfigGroup,
    label: Label,
    timer: Timer,
}

impl Component for Clock {
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

impl Clock {
    pub fn init(
        config: ConfigGroup, container: &gtk::Box,
    ) -> Box<Self> {

        let label = Label::new(None);

        // TODO: init_widget
        container.add(&label);
        label.show();

        // get config
        let symbols = SymbolFmt::new(config.get_str_or("format", "{timestamp}"));
        let timestamp = config.get_str_or("timestamp", "%Y-%m-%d %H:%M:%S").to_string();
        let interval = config.get_int_or("interval", 1).max(1);

        // start timer
        let tick = clone!(label move || {
            let time = &format!("{}", Local::now().format(&timestamp));
            label.set_markup(&symbols.format(|sym| match sym {
                "timestamp" => time.to_string(),
                _ => sym.to_string(),
            }));
            gtk::Continue(true)
        });
        tick();
        let timer = Timer::add_seconds(interval as u32, tick);

        Box::new(Clock { config, label, timer })
    }
}
