use gtk;
use gtk::prelude::*;
use gtk::Label;
use bar::Bar;
use components::Component;
use config::ConfigGroup;
use wm::events::{Event, EventValue, EventId};
use wm::WMUtil;
use util::SymbolFmt;
use glib::markup_escape_text;

pub struct Mode {
    config: ConfigGroup,
    label: Label,
    event_id: EventId,
    wm_util: WMUtil,
}

impl Component for Mode {
    fn get_config(&self) -> &ConfigGroup {
        &self.config
    }
    fn show(&mut self) {
        self.label.show();
    }
    fn hide(&mut self) {
        self.label.hide();
    }
    fn destroy(&self) {
        self.wm_util.remove_listener(Event::Mode, self.event_id);
        self.label.destroy();
    }
}

impl Mode {
    pub fn init(config: ConfigGroup, bar: &mut Bar, container: &gtk::Box) {
        let label = Label::new(None);
        super::init_widget(&label, &config, bar, container);

        let symbols = SymbolFmt::new(config.get_str_or("format", "{mode}"));

        let event_id = bar.wm_util.add_listener(Event::Mode, clone!(label
            move |event_opt| {
                if let Some(EventValue::String(mode)) = event_opt {
                    let is_default = mode == "default";

                    if is_default {
                        label.hide();
                    } else {
                        label.show();
                        let mode = &mode;
                        label.set_markup(&symbols.format(|sym| {
                            match sym {
                                "mode" => markup_escape_text(mode),
                                _ => sym.to_string(),
                            }
                        }));
                    }
                }
            }
        ));

        let wm_util = bar.wm_util.clone();
        bar.add_component(Box::new(Mode {
            config,
            label,
            wm_util,
            event_id,
        }));
    }
}
