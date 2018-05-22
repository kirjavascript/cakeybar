// extern crate pnet;

extern crate ifaces;

use super::{Component, Bar, gtk, ComponentConfig};
use gtk::prelude::*;
use gtk::{Label};

// use self::pnet::datalink;

pub struct Network { }

impl Component for Network {
    fn init(container: &gtk::Box, config: &ComponentConfig, _bar: &Bar) {
        let label = Label::new(None);
        Network::init_widget(&label, config);
        container.add(&label);

        let interface = String::from(config.get_str_or("interface", "null"));

        let label_tick_clone = label.clone();
        let tick = move || {
    for iface in
        ifaces::Interface::get_all().unwrap()
            .into_iter() {
                println!("{}\t{:?}\t{:?}", iface.name, iface.kind, iface.addr);
            }
            // let interfaces = datalink::interfaces();
            // if let Some(iface) = interfaces.iter().find(|x| x.name == interface) {
            //     label_tick_clone.set_text(&format!("{}", iface.ips[0].ip()));
            // }
            gtk::Continue(true)
        };

        let interval = config.get_int_or("interval", 5);
        tick();
        gtk::timeout_add_seconds(interval as u32, tick);
    }
}
