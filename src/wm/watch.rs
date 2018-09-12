use notify::{Watcher, RecursiveMode, raw_watcher, RawEvent, op};
use std::sync::mpsc;
use crossbeam_channel as channel;
use std::thread;
use util::Timer;
use wm::WMUtil;
use gtk;

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

    let (s, r) = channel::unbounded();
    let (s_dead, r_dead) = channel::unbounded();

    thread::spawn(move || {
        // TODO: clean up unwrap
        let (tx, rx) = mpsc::channel();
        let mut watcher = raw_watcher(tx).unwrap();
        watcher.watch(&filename, RecursiveMode::Recursive).unwrap();
        watcher.watch(&theme, RecursiveMode::Recursive).unwrap();
        loop {
            match rx.try_recv() {
               Ok(RawEvent{path: Some(path), op: Ok(op), .. }) => {
                   if op == op::CLOSE_WRITE {
                       if let Some(filename) = path.file_name() {
                           info!("wrote {}", filename.to_string_lossy());
                       }
                       if path.to_string_lossy().into_owned() == theme {
                           s.send(WriteType::Theme);
                       } else {
                           s.send(WriteType::Config);
                       }
                   }
               },
               Ok(event) => println!("broken event: {:?}", event),
               Err(e) => println!("watch error: {:?}", e),
            }
        }
    });

    // tx_clone.send(());

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
