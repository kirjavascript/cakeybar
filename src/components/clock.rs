use crate::components::{Component, ComponentParams};
use chrono::Local;
use gtk::prelude::*;
use gtk::Label;
use crate::util::{SymbolFmt, Timer};

pub struct Clock {
    label: Label,
    timer: Timer,
}

impl Component for Clock {
    fn destroy(&self) {
        self.timer.remove();
        self.label.destroy();
    }
}

impl Clock {
    pub fn init(params: ComponentParams) {
        let ComponentParams { config, window, container, .. } = params;
        let label = Label::new(None);
        super::init_widget(&label, &config, &window, container);
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

        window.add_component(Box::new(Clock {
            label,
            timer,
        }));
    }
}
