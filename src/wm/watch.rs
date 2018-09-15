use std::sync::mpsc;
use crossbeam_channel as channel;
use std::{thread, time};
use util::Timer;
use wm::WMUtil;
use gtk;

use inotify;
use std::env;
use inotify::{
    Inotify,
    WatchMask,
};

enum WriteType {
    Config,
    Theme,
}

pub struct Unwatcher {
    timer: Timer,
    sender: channel::Sender<()>,
}

impl Unwatcher {
    pub fn unwatch(&self) {
        self.timer.remove();
        self.sender.send(());
    }
}

pub fn watch(wm_util: &WMUtil, filename: String, theme: String) -> Unwatcher {
    // TODO: pass in config

    let (s, r) = channel::unbounded();
    let (s_dead, r_dead) = channel::unbounded();

    thread::spawn(move || {
        let mut inotify = Inotify::init().unwrap();

        let mut watchers = Vec::new();

        match inotify.add_watch(&filename, WatchMask::CLOSE_WRITE) {
            Ok(watcher) => watchers.push((&filename, watcher)),
            Err(err) => error!("failed to watch {}: {}", &filename, err),
        }
        match inotify.add_watch(&theme, WatchMask::CLOSE_WRITE) {
            Ok(watcher) => watchers.push((&theme, watcher)),
            Err(err) => error!("failed to watch {}: {}", &theme, err),
        }

        let mut buffer = [0; 1024];
        loop {
            let events = inotify.read_events(&mut buffer)
                .expect("error reading events");
            for event in events {
                let wd_opt = watchers.iter().find(|(_, wd)| wd == &event.wd);
                if let Some((path, _)) = wd_opt {
                    let fragments = path.split("/").collect::<Vec<&str>>();
                    if let Some(head) = fragments.last() {
                        info!("wrote {}", head);
                    }
                    if path == &&filename {
                        s.send(WriteType::Config);
                    } else {
                        s.send(WriteType::Theme);
                    }
                }
            }
            if let Some(_) = r_dead.try_recv() {
                error!("rip");
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

    Unwatcher { timer, sender: s_dead }
}
