use inotify::{Inotify, WatchMask};
use crossbeam_channel as channel;
use std::{thread, time};
use crate::bar::Bar;
use crate::components::Component;
use crate::config::ConfigGroup;
use gtk::prelude::*;
use crate::util::{read_file, SymbolFmt, Timer};
use gtk::Label;

pub struct Backlight {
    label: Label,
    timer: Timer,
    watcher: channel::Sender<()>,
}

impl Component for Backlight {
    fn destroy(&self) {
        self.label.destroy();
        self.timer.remove();
        self.watcher.send(());
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
        match get_value("brightness") {
            Ok(initial) => {
                let label = Label::new(None);
                super::init_widget(&label, &config, bar, container);
                label.show();

                let (s, r) = channel::unbounded();
                let (s_dead, r_dead) = channel::unbounded();

                let max = get_value("max_brightness").unwrap_or(initial);

                s.send((initial/max)*100.);

                thread::spawn(move || {
                    let mut inotify = Inotify::init().unwrap();

                    let wd_res =inotify.add_watch(
                        "/sys/class/backlight/intel_backlight/brightness",
                        WatchMask::MODIFY,
                    );

                    match wd_res {
                        Ok(wd) => {
                            let mut buffer = [0; 1024];
                            loop {
                                let events = inotify.read_events(&mut buffer)
                                    .expect("error reading events");
                                for _ in events {
                                    let now = get_value("brightness").unwrap();
                                    s.send((now/max)*100.);
                                }
                                if r_dead.try_recv().is_some() {
                                    inotify.rm_watch(wd).ok();
                                    break;
                                } else {
                                    thread::sleep(time::Duration::from_millis(50));
                                }
                            }
                        },
                        Err(err) => {
                            error!("{}", err.to_string());
                        },
                    }
                });

                let symbols = SymbolFmt::new(config.get_str_or("format", "{percent}"));

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
                    label,
                    timer,
                    watcher: s_dead,
                }));

            },
            Err(err) => {
                error!("reading brightness {}", err);
            },
        }
    }
}
