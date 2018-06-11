use super::{Component, Bar, gtk, ComponentConfig};
use gtk::prelude::*;
use gtk::{Orientation};

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

        let wrapper = gtk::Box::new(Orientation::Horizontal, 0);
        Tray::init_widget(&wrapper, &config);
        container.add(&wrapper);
        // wrapper.connect_size_allocate(move |_, rect| {
        //     println!("{:#?}", rect);
        // });
        wrapper.show();
        wrapper.set_size_request(icon_size as i32, 5);

        gtk::idle_add(enclose!(container move || {
            let (tx_ipc, rx_ipc) = ::tray::ipc::get_server();
            ::tray::as_subprocess();

            tx_ipc.send("component".to_string());
            gtk::Continue(false)
        }));
    }
}
