use notify::{Watcher, RecursiveMode, raw_watcher, RawEvent};
use std::sync::mpsc;
use crossbeam_channel as channel;
use std::thread;
use util::Timer;
use wm::WMUtil;
use gtk;

pub fn watch(wm_util: &WMUtil, filename: String, theme: String) {

    let (s, r) = channel::unbounded();

    thread::spawn(move || {
        let (tx, rx) = mpsc::channel();
        let mut watcher = raw_watcher(tx).unwrap();
        watcher.watch(filename, RecursiveMode::Recursive).unwrap();
        watcher.watch(theme, RecursiveMode::Recursive).unwrap();
        loop {
            match rx.recv() {
               Ok(RawEvent{path: Some(path), op: Ok(op), cookie}) => {
                   s.send(format!("{:?} {:?} ({:?})", op, path, cookie));
               },
               Ok(event) => println!("broken event: {:?}", event),
               Err(e) => println!("watch error: {:?}", e),
            }
        }
    });

    let timer = Timer::add_ms(50, clone!(wm_util move || {
        if let Some(msg) = r.try_recv() {
            // println!("{:#?}", msg);
            wm_util.load_theme(None);
            // wm_util.reload_config(None);
            // TODO: fix relayout
        }
        gtk::Continue(true)
    }));
}
