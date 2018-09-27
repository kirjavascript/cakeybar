use bar::Bar;
use components::Component;
use config::ConfigGroup;
use glib::markup_escape_text;
use gtk;
use gtk::prelude::*;
use gtk::Label;
use util::SymbolFmt;

use wm::events::{Event, EventId, EventValue};
use wm::WMUtil;

pub struct Window {
    config: ConfigGroup,
    label: Label,
    event_id: EventId,
    wm_util: WMUtil,
}

impl Component for Window {
    fn get_config(&self) -> &ConfigGroup {
        &self.config
    }
    fn show(&self) {
        self.label.show();
    }
    fn hide(&self) {
        self.label.hide();
    }
    fn destroy(&self) {
        self.wm_util.remove_listener(Event::Window, self.event_id);
        self.label.destroy();
    }
}

impl Window {
    pub fn init(config: ConfigGroup, bar: &mut Bar, container: &gtk::Box) {
        let label = Label::new(None);
        super::init_widget(&label, &config, bar, container);
        label.show();

        let trunc = config.get_int_or("truncate", 100) as usize;
        let symbols = SymbolFmt::new(config.get_str_or("format", "{title}"));

        let event_id = bar.wm_util.add_listener(Event::Window, clone!(label
            move |event_opt| {
                if let Some(EventValue::String(name)) = event_opt {
                    let name = &name;
                    if name.len() == 0 {
                        label.set_markup(name);
                    } else {
                        let output = symbols.format(|sym| match sym {
                            "title" => {
                                if name.chars().count() > trunc {
                                    let parsed = name
                                        .char_indices()
                                        .filter(|x| x.0 <= trunc)
                                        .fold("".to_string(), |acc, cur| {
                                            acc + &cur.1.to_string()
                                        });
                                    format!("{}…", markup_escape_text(&parsed))
                                } else {
                                    markup_escape_text(name)
                                }
                            },
                            _ => sym.to_string(),
                        });
                        label.set_markup(&output);
                    }
                }
            }
        ));

        let wm_util = bar.wm_util.clone();
        bar.add_component(Box::new(Window {
            config,
            label,
            wm_util,
            event_id,
        }));
    }
}
