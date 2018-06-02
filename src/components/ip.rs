use systemstat::{System, Platform};
use systemstat::data::{IpAddr, Network};

use super::{Component, Bar, gtk, ComponentConfig};
use gtk::prelude::*;
use gtk::{Label};

pub struct IP {}

impl Component for IP {
    fn init(container: &gtk::Box, config: &ComponentConfig, _bar: &Bar) {
        let label = Label::new(None);
        IP::init_widget(&label, config);
        container.add(&label);
        label.show();

        let interface = String::from(config.get_str_or("interface", "lo"));
        let ipv6 = config.get_bool_or("ipv6", false);

        let sys = System::new();

        let label_tick_clone = label.clone();
        let tick = move || {
            if let Ok(interfaces) = sys.networks() {
                let mut iterface_opt = interfaces.iter().find(|x| x.0 == &interface);
                if let Some((_name, iface)) = iterface_opt {
                    let ip_opt = IP::get_ip_from_network(iface, ipv6);
                    if let Some(ip) = ip_opt {
                        label_tick_clone.set_text(&ip);
                    } else {
                        // if we dont find addresses, see if ANY interface has them
                        let other_opt = interfaces.iter().find(|x| {
                            IP::get_ip_from_network(x.1, ipv6).is_some()
                        });
                        if let Some((_name, iface)) = other_opt {
                            let ip_opt = IP::get_ip_from_network(iface, ipv6);
                            if let Some(ip) = ip_opt {
                                label_tick_clone.set_text(&ip);
                            }
                        }
                    }
                }
            }
            gtk::Continue(true)
        };

        let interval = config.get_int_or("interval", 5);
        tick();
        gtk::timeout_add_seconds(interval as u32, tick);
    }


}

impl IP {
     fn get_ip_from_network(interface: &Network, ipv6: bool) -> Option<String> {
         for addr in interface.addrs.iter() {
             if let IpAddr::V6(ip) = addr.addr {
                 if ipv6 {
                     return Some(format!("{}", ip));
                 }
             }
             else if let IpAddr::V4(ip) = addr.addr {
                 if !ipv6 {
                     return Some(format!("{}", ip));
                 }
             }
         }
         None
     }
}
