use super::{Component, Bar, gtk, ConfigGroup};
use gtk::prelude::*;
use gtk::{Label};
use wm::events::{Event, EventValue};
use util::SymbolFmt;

pub struct Mode;

impl Component for Mode {
    fn init(container: &gtk::Box, config: &ConfigGroup, bar: &Bar){
        let label = Label::new(None);
        Self::init_widget(&label, container, config, bar);

        let symbols = SymbolFmt::new(config.get_str_or("format", "{mode}"));

        bar.wm_util.add_listener(Event::Mode, clone!(label
            move |event_opt| {
                if let Some(EventValue::String(mode)) = event_opt {
                    let is_default = mode == "default";

                    if is_default {
                        label.hide();
                    } else {
                        label.show();
                        let mode = &mode;
                        label.set_text(&symbols.format(|sym| {
                            match sym {
                                "mode" => mode.to_string(),
                                _ => sym.to_string(),
                            }
                        }));
                    }
                }
            }
        ));
    }
}
