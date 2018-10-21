use inotify::{Inotify, WatchMask};
use crossbeam_channel as channel;
use std::{thread, time};
use bar::Bar;
use components::Component;
use config::ConfigGroup;
use gtk;
use gdk;
use gtk::prelude::*;
use util::{read_file, SymbolFmt, Timer};
use gtk::Label;

pub struct Backlight {
    config: ConfigGroup,
}

impl Component for Backlight {
    fn get_config(&self) -> &ConfigGroup {
        &self.config
    }
    fn show(&self) {
        // self.image.show();
    }
    fn hide(&self) {
        // self.image.hide();
    }
    fn destroy(&self) {
        // self.image.destroy();
    }
}

impl Backlight {
    pub fn init(config: ConfigGroup, bar: &mut Bar, container: &gtk::Box) {

        let label = Label::new(None);
        super::init_widget(&label, &config, bar, container);
        label.show();

        // get max brightness initially
        // inotify brightness

        // let configfile = config.get_filename();
        // let theme = config.get_theme();

        let (s, r) = channel::unbounded();
        // let (s_dead, r_dead) = channel::unbounded();

        let brightness: &str = "/sys/class/backlight/intel_backlight/brightness";

        thread::spawn(move || {
            let mut inotify = Inotify::init().unwrap();

            let wd_res = inotify.add_watch(brightness, WatchMask::MODIFY);
            println!("{:#?}", wd_res);


            let max = read_file("/sys/class/backlight/intel_backlight/max_brightness").unwrap().parse::<f32>().unwrap();

            let mut buffer = [0; 1024];
            loop {
                let events = inotify.read_events(&mut buffer)
                    .expect("error reading events");
                for event in events {
                    let now = read_file(brightness).unwrap().parse::<f32>().unwrap();
                    s.send((now/max)*100.);
                }
            }
        });

        let timer = Timer::add_ms(50, clone!(label move || {
            if let Some(pct) = r.try_recv() {
                label.set_markup(&format!("{}%", pct as u32));
            }
            gtk::Continue(true)
        }));

        bar.add_component(Box::new(Backlight {
            config,
        }));
    }
}
