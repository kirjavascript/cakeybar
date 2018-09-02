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

        clock.start_timer();

        // check show/hide removes timeout
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
//
// use super::{Component, Bar, gtk, ConfigGroup};
// use gtk::prelude::*;
// use gtk::{Label};
// use chrono::Local;
// use util::SymbolFmt;

// pub struct Clock;


// fn current_time(format: String) -> String {
//     return format!("{}", Local::now().format(&timestamp));
// }

// impl Component for Clock {
//     fn init(container: &gtk::Box, config: &ConfigGroup, bar: &Bar) {
//         let label = Label::new(None);

//         let timestamp = config.get_str_or("timestamp", "%Y-%m-%d %H:%M:%S").to_string();
//         let symbols = SymbolFmt::new(config.get_str_or("format", "{timestamp}"));

//         let tick = clone!(label move || {
//             let time = &current_time(timestamp.clone());
//             label.set_markup(&symbols.format(|sym| {
//                 match sym {
//                     "timestamp" => time.to_string(),
//                     _ => sym.to_string(),
//                 }
//             }));
//             gtk::Continue(true)
//         });

//         let interval = config.get_int_or("interval", 1).max(1);
//         tick();
//         gtk::timeout_add_seconds(interval as u32, tick);

//         label.show();

//         Self::init_widget(&label, container, config, bar);
//     }
// }
