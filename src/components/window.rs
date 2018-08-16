use super::{Component, Bar, gtk, ComponentConfig};
use gtk::prelude::*;
use gtk::{Label};
use util::SymbolFmt;

use wm::events::{Event, EventValue};

pub struct Window;

impl Component for Window {
    fn init(container: &gtk::Box, config: &ComponentConfig, bar: &Bar){
        let label = Label::new(None);

        Self::init_widget(&label, container, config, bar);
        label.show();

        let trunc = config.get_int_or("truncate", 100) as usize;
        let symbols = SymbolFmt::new(config.get_str_or("format", "{title}"));

        bar.wm_util.add_listener(Event::Window, clone!(label
            move |event_opt| {
                if let Some(EventValue::String(name)) = event_opt {
                    let name = &name;
                    if name.len() == 0 {
                        label.set_markup(name);
                    } else {
                        let output = symbols.format(|sym| {
                            match sym {
                                "title" => {
                                    if name.chars().count() > trunc {
                                        let parsed = name
                                            .char_indices()
                                            .filter(|x| x.0 <= trunc)
                                            .fold("".to_string(), |acc, cur| {
                                                acc + &cur.1.to_string()
                                            });
                                        format!("{}…", parsed)
                                    } else {
                                        name.to_string()
                                    }
                                },
                                _ => sym.to_string(),
                            }
                        });
                        label.set_markup(&output);
                    }
                }
            }
        ));
    }
}
