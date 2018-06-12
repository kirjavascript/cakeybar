use super::{Component, Bar, gtk, ComponentConfig};
use gtk::prelude::*;
use gtk::{Orientation};
use gdk::{WindowExt};

use ::tray::ipc::Message;

pub struct Tray { }

// mutable statics should be safe within the same thread
static mut TRAY_LOADED: bool = false;

impl Component for Tray {
    fn init(container: &gtk::Box, config: &ComponentConfig, bar: &Bar) {
        if unsafe { !TRAY_LOADED } {
            unsafe { TRAY_LOADED = true; }
            Tray::load(container, config, bar);
        }
        else {
            eprintln!("Tray component is already loaded");
        }
    }
}

impl Tray {
    fn load(container: &gtk::Box, config: &ComponentConfig, _bar: &Bar) {
        let bg = config.get_str_or("background_color", "#000000");
        let icon_size = config.get_int_or("icon_size", 20);
        let bg_hex = match u32::from_str_radix(&bg[1..], 16) {
            Ok(val) => val,
            Err(_) => 0,
        };

        // extra surrounding base widget added for margins, etc
        let wrapper = gtk::Box::new(Orientation::Horizontal, 0);
        let base_widget = gtk::Box::new(Orientation::Horizontal, 0);
        Tray::init_widget(&base_widget, &config);
        base_widget.add(&wrapper);
        container.add(&base_widget);
        base_widget.show_all();

        gtk::idle_add(enclose!(wrapper move || {
            let (tx_ipc, rx_ipc) = ::tray::ipc::get_server();
            ::tray::as_subprocess();

            tx_ipc.send(Message::BgColor(bg_hex));
            tx_ipc.send(Message::IconSize(icon_size as u16));

            wrapper.connect_size_allocate(move |c, rect| {
                let w = c.get_window().unwrap();
                let (_zo, xo, yo) = w.get_origin();
                let y = (yo + (rect.y + ((rect.height - (icon_size as i32))/2))) as u32;
                let x = (xo + rect.x) as u32;
                tx_ipc.send(Message::Move(x, y));
            });

            gtk::timeout_add(10, enclose!(wrapper move || {
                if let Ok(msg) = rx_ipc.try_recv() {
                    match msg {
                        Message::Width(w) => {
                            wrapper.set_size_request(w as i32, icon_size as i32);
                        },
                        _ => {},
                    }
                }
                gtk::Continue(true)
            }));
            gtk::Continue(false)
        }));
    }
}
