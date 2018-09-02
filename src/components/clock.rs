use gtk;
use gtk::prelude::*;
use gtk::Label;
    use glib_sys;
    use glib::translate::ToGlib;
use config::{ConfigGroup, Property};
use components::Component;
use bar::Bar;
use chrono::Local;
use util::SymbolFmt;

// TODO: impl util::Timer(u32) Timer::from(SourceId)

pub struct Clock {
    config: ConfigGroup,
    label: Label,
    timer: Option<u32>,
}

impl Component for Clock {
    fn show(&self) {
        self.label.show();
    }
    fn hide(&self) {
        self.label.hide();
    }
    fn destroy(&self) {
        if let Some(timer) = self.timer {
            unsafe {
                glib_sys::g_source_remove(timer);
            }
        }
        self.label.destroy();
    }
}

impl Clock {
    pub fn init(
        config: ConfigGroup, bar: &Bar, container: &gtk::Box,
    ) -> Box<Self> {

        let label = Label::new(None);

        let mut clock = Clock { config, label, timer: None };

        // TODO: init_widget
        container.add(&clock.label);
        clock.label.show();

        // open closure inside here?

        clock.start_timer();

        Box::new(clock)
    }

    fn start_timer(&mut self) {
        let symbols = SymbolFmt::new(self.config.get_str_or("format", "{timestamp}"));
        let timestamp = self.config.get_str_or("timestamp", "%Y-%m-%d %H:%M:%S").to_string();
        let label = self.label.clone();
        let tick = move || {
            let time = &format!("{}", Local::now().format(&timestamp));
;
            label.set_markup(&symbols.format(|sym| {
                match sym {
                    "timestamp" => time.to_string(),
                    _ => sym.to_string(),
                }
            }));
            gtk::Continue(true)
        };
        tick();
        let interval = self.config.get_int_or("interval", 1).max(1);
        let timer = gtk::timeout_add_seconds(interval as u32, tick);
        self.timer = Some(timer.to_glib());
    }
}
