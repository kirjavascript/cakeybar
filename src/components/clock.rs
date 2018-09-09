use bar::Bar;
use chrono::Local;
use components::Component;
use config::ConfigGroup;
use gtk;
use gtk::prelude::*;
use gtk::Label;
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
    pub fn init(config: ConfigGroup, bar: &mut Bar, container: &gtk::Box) {
        let label = Label::new(None);
        super::init_widget(&label, &config, bar, container);
        label.show();

        // get config
        let symbols = SymbolFmt::new(config.get_str_or("format", "{timestamp}"));
        let timestamp = config
            .get_str_or("timestamp", "%Y-%m-%d %H:%M:%S")
            .to_string();
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
        let timer = Timer::add_seconds(interval as u32, tick);

        bar.add_component(Box::new(Clock {
            config,
            label,
            timer,
        }));
    }
}
