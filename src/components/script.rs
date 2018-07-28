use super::{Component, Bar, gtk, ComponentConfig};
use config::Property;
use gtk::prelude::*;
use gtk::{Label};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;
use std::sync::mpsc;

pub struct Script { }

impl Component for Script {
    fn init(container: &gtk::Box, config: &ComponentConfig, bar: &Bar) {
        if let Some(&Property::String(ref src)) = config.properties.get("src") {

            let (tx, rx) = mpsc::channel();
            let interval = config.get_int_or("interval", 5).max(1);

            thread::spawn(clone!((src, interval) move || {
                loop {
                    let child = Command::new("/bin/sh")
                        .arg("-c")
                        .arg(&src)
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped())
                        .output();

                    if let Ok(output) = child {
                        let stdout = String::from_utf8_lossy(&output.stdout);
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        tx.send((format!("{}", stdout), format!("{}", stderr))).ok();
                    }

                    thread::sleep(Duration::from_secs(interval as u64));
                }

            }));

            let label = Label::new(None);
            gtk::timeout_add_seconds(1, clone!((label, src) move || {
                if let Ok((stdout, stderr)) = rx.try_recv() {
                    label.set_text(&stdout.trim());
                    if stderr.len() > 0 {
                        error!("{}: {}", src, stderr.trim());
                    }
                }
                gtk::Continue(true)
            }));

            label.show();
            Self::init_widget(&label, container, config, bar);

        } else {
            warn!("src property missing from #{}", config.name);
        }
    }
}
