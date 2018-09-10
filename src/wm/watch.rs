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

pub fn watch(wm_util: &WMUtil, filename: String, theme: String) {

    let (s, r) = channel::unbounded();

    thread::spawn(move || {
        let (tx, rx) = mpsc::channel();
        let mut watcher = raw_watcher(tx).unwrap();
        watcher.watch(&filename, RecursiveMode::Recursive).unwrap();
        watcher.watch(&theme, RecursiveMode::Recursive).unwrap();
        loop {
            match rx.recv() {
               Ok(RawEvent{path: Some(path), op: Ok(op), .. }) => {
                   if op == op::CLOSE_WRITE {
                       if let Some(filename) = path.file_name() {
                           info!("wrote {}...", filename.to_string_lossy());
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

    let _timer = Timer::add_ms(50, clone!(wm_util move || {
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

    // TODO: cleanup
}
