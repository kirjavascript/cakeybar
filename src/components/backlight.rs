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
    label: Label,
    timer: Timer,
}

impl Component for Backlight {
    fn get_config(&self) -> &ConfigGroup {
        &self.config
    }
    fn show(&self) {
        self.label.show();
    }
    fn hide(&self) {
        self.label.hide();
    }
    fn destroy(&self) {
        self.label.destroy();
        self.timer.remove();
    }
}

fn get_value(name: &str) -> Result<f32, String> {
    read_file(&format!("/sys/class/backlight/intel_backlight/{}", name))
        .map_err(|e| e.to_string())?
        .parse::<f32>()
        .map_err(|e| e.to_string())
}

impl Backlight {
    pub fn init(config: ConfigGroup, bar: &mut Bar, container: &gtk::Box) {
        let label = Label::new(None);
        super::init_widget(&label, &config, bar, container);
        label.show();


        match get_value("brightness") {
            Ok(initial) => {
                let (s, r) = channel::unbounded();
                let max = get_value("max_brightness").unwrap_or(initial);

                // messages - rip/err/symbols

                s.send((initial/max)*100.);

                thread::spawn(move || {
                    let mut inotify = Inotify::init().unwrap();

                    let wd_res = inotify.add_watch(
                        "/sys/class/backlight/intel_backlight/brightness",
                        WatchMask::MODIFY,
                    );
                    println!("{:#?}", wd_res); // TODO



                    let mut buffer = [0; 1024];
                    loop {
                        let events = inotify.read_events(&mut buffer)
                            .expect("error reading events"); // TODO
                        for _ in events {
                            let now = get_value("brightness").unwrap();
                            s.send((now/max)*100.);
                        }
                        // if message, else
                        thread::sleep(time::Duration::from_millis(50));
                    }
                });

                let symbols = SymbolFmt::new(config.get_str_or("format", "{pct}"));

                let timer = Timer::add_ms(50, clone!(label move || {
                    if let Some(pct) = r.try_recv() {
                        label.set_markup(&symbols.format(|sym| match sym {
                            "pct" => format!("{:?}%", pct as u32),
                            _ => sym.to_string(),
                        }));
                    }
                    gtk::Continue(true)
                }));

                bar.add_component(Box::new(Backlight {
                    config,
                    label,
                    timer,
                }));

            },
            Err(err) => {
                error!("reading brightness {}", err);
            },
        }
    }
}
