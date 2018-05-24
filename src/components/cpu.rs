// use systemstat::{System, Platform};
// use systemstat::data::IpAddr;

use super::{Component, Bar, gtk, ComponentConfig};
use gtk::prelude::*;
use gtk::{Label};

// use probes::{cpu, network, load};

pub struct CPU { }

impl Component for CPU {
    fn init(container: &gtk::Box, config: &ComponentConfig, _bar: &Bar) {
        let label = Label::new(None);
        CPU::init_widget(&label, config);
        container.add(&label);
        label.show();

        // let sys = System::new();

        let label_tick_clone = label.clone();
        let tick = move || {
            // let last_measurement_res = sys.cpu_load();
            // match last_measurement_res {
            //     Ok(load) => {

            //         gtk::timeout_add_seconds(1, move || {
            //                 println!("{:#?}", load.done());

            //             gtk::Continue(false)
            //         });
            //     },
            //     Err(e) => {
            //         eprintln!("{:?}", e);
            //     },
            // }

            // let cpu = network::read();
            // match cpu {
            //     Ok(info) => {
            //         let received = info.interfaces.get("enp3s0").unwrap().received;
            //         let s = format!("{}kb", received);
            //         label_tick_clone.set_text(&s);
            //     },
            //     Err(err) => {
            //         eprintln!("{}", err);
            //     },
            // }

                    label_tick_clone.set_text(&"CPU: ?");
            gtk::Continue(true)
        };

        let interval = config.get_int_or("interval", 1);
        tick();
        gtk::timeout_add_seconds(interval as u32, tick);
    }
}
