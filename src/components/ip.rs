use bar::Bar;
use components::Component;
use config::ConfigGroup;
use gtk;
use gtk::prelude::*;
use util::{LabelGroup, SymbolFmt, Timer};

use systemstat::data::{IpAddr, Network};
use systemstat::{Platform, System};

pub struct IP {
    config: ConfigGroup,
    wrapper: gtk::Box,
    timer: Timer,
}

impl Component for IP {
    fn get_config(&self) -> &ConfigGroup {
        &self.config
    }
    fn show(&mut self) {
        self.wrapper.show();
    }
    fn hide(&mut self) {
        self.wrapper.hide();
    }
    fn destroy(&self) {
        self.timer.remove();
        self.wrapper.destroy();
    }
}

impl IP {
    pub fn init(config: ConfigGroup, bar: &mut Bar, container: &gtk::Box) {
        let label_group = LabelGroup::new();
        super::init_widget(&label_group.wrapper, &config, bar, container);

        let interfaces = config.get_string_vec("interfaces");

        let should_include =
            move |s: &str| interfaces.len() == 0 || interfaces.contains(&&s.to_string());

        let symbols = SymbolFmt::new(config.get_str_or("format", "{ipv4}"));

        let sys = System::new();

        let tick = clone!(label_group move || {
            if let Ok(interfaces) = sys.networks() {
                let mut labels = Vec::new();
                for interface in interfaces {
                    if should_include(&interface.0) {
                        let text = symbols.format(|sym| match sym {
                            "name" => interface.0.clone(),
                            "ipv4" => {
                                Self::get_addr_from_network(
                                    &interface.1,
                                    false,
                                    )
                            },
                            "ipv6" => {
                                Self::get_addr_from_network(
                                    &interface.1,
                                    true,
                                )
                            },
                            _ => sym.to_string(),
                        });
                        labels.push(text);
                    }
                }
                label_group.set(&labels);
            }
            gtk::Continue(true)
        });

        let interval = config.get_int_or("interval", 3).max(1);
        let timer = Timer::add_seconds(interval as u32, tick);

        bar.add_component(Box::new(IP {
            config,
            wrapper: label_group.wrapper,
            timer,
        }));
    }

    fn get_addr_from_network(interface: &Network, ipv6: bool) -> String {
        for addr in interface.addrs.iter() {
            if let IpAddr::V6(ip) = addr.addr {
                if ipv6 {
                    return format!("{}", ip);
                }
            } else if let IpAddr::V4(ip) = addr.addr {
                if !ipv6 {
                    return format!("{}", ip);
                }
            }
        }
        format!("no IPv{}", if ipv6 { 6 } else { 4 })
    }
}
