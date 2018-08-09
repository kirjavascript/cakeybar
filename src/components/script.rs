use super::{Component, Bar, gtk, ComponentConfig};
use config::Property;
use gtk::prelude::*;
use gtk::{Label};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::io::Error;
use util::SymbolFmt;

pub struct Script { }

fn get_output(src: &str) -> Result<(String, String, i32), Error> {
    let output = Command::new("/bin/sh")
        .arg("-c")
        .arg(&src)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let code = output.status.code().unwrap_or(0);
    Ok((stdout, stderr, code))
}

impl Component for Script {
    fn init(container: &gtk::Box, config: &ComponentConfig, bar: &Bar) {
        if let Some(&Property::String(ref src)) = config.properties.get("src") {

            let (tx, rx) = mpsc::channel();
            let interval = config.get_int_or("interval", 3).max(1);
            let symbols = SymbolFmt::new(config.get_str_or("format", "{stdout}"));

            if let Ok(output) = get_output(&src) {
                tx.send(output).ok();
            }

            thread::spawn(clone!((src, interval) move || {
                loop {
                    if let Ok(output) = get_output(&src) {
                        tx.send(output).ok();
                    }
                    thread::sleep(Duration::from_secs(interval as u64));
                }

            }));

            let label = Label::new(None);
            let tick = clone!((label, src) move || {
                if let Ok((ref stdout, ref stderr, code)) = rx.try_recv() {
                    label.set_text(&symbols.format(|sym| {
                        match sym {
                            "stdout" => stdout.to_string(),
                            "stderr" => stderr.to_string(),
                            "code" => code.to_string(),
                            _ => sym.to_string(),
                        }
                    }));
                }
                gtk::Continue(true)
            });
            tick();
            gtk::timeout_add_seconds(1, tick);

            label.show();
            Self::init_widget(&label, container, config, bar);

        } else {
            warn!("src property missing from #{}", config.name);
        }
    }
}
