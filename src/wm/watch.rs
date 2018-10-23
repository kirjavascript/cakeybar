use inotify::{Inotify, WatchMask};
use crossbeam_channel as channel;
use std::{thread, time};
use util::Timer;
use wm::WMUtil;
use config::Config;
use gtk;

enum WriteType {
    Config,
    Theme,
}

pub struct Watcher {
    timer: Timer,
    sender: channel::Sender<()>,
}

impl Watcher {
    pub fn new(wm_util: &WMUtil, config: &Config) -> Watcher {
        let configfile = config.get_filename();
        let theme = config.get_theme();

        let (s, r) = channel::unbounded();
        let (s_dead, r_dead) = channel::unbounded();

        thread::spawn(move || {
            let mut inotify = Inotify::init().unwrap();

            let file_wd = match inotify.add_watch(&configfile, WatchMask::CLOSE_WRITE) {
                Ok(watcher) => Some((&configfile, watcher)),
                Err(err) => { error!("failed to watch {}: {}", &configfile, err); None },
            };
            let theme_wd = match inotify.add_watch(&theme, WatchMask::CLOSE_WRITE) {
                Ok(watcher) => Some((&theme, watcher)),
                Err(err) => { error!("failed to watch {}: {}", &theme, err); None },
            };

            let mut buffer = [0; 1024];
            loop {
                let events = inotify.read_events(&mut buffer)
                    .expect("error reading events");
                for event in events {
                    if let &Some((_, ref wd)) = &file_wd {
                        if wd == &event.wd {
                            s.send(WriteType::Config);
                            info!("updated {}", &configfile);
                        } else {
                            s.send(WriteType::Theme);
                            info!("updated {}", &theme);
                        }
                    } else if let Some(_) = &theme_wd {
                        // doesn't work when there's no valid config file
                        // ...which should never happen
                        error!("this should never happen");
                    }
                }
                if r_dead.try_recv().is_some() {
                    // remove watchers
                    if let Some((_, wd)) = file_wd {
                        inotify.rm_watch(wd).ok();
                    }
                    if let Some((_, wd)) = theme_wd {
                        inotify.rm_watch(wd).ok();
                    }
                    // end thread
                    break;
                } else {
                    thread::sleep(time::Duration::from_millis(50));
                }
            }
        });

        let timer = Timer::add_ms(50, clone!(wm_util move || {
            if let Some(wtype) = r.try_recv() {
                match wtype {
                    WriteType::Config => {
                        wm_util.reload_config(None);
                    },
                    WriteType::Theme => {
                        wm_util.load_theme(None);
                    },
                }
            }
            gtk::Continue(true)
        }));

        Watcher { timer, sender: s_dead }
    }
    pub fn unwatch(&self) {
        self.timer.remove();
        self.sender.send(());
    }
}
