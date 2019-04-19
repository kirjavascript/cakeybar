use crate::components::{Component, ComponentParams};
use glib::markup_escape_text;
use gtk;
use gtk::prelude::*;
use gtk::{Label, Orientation};
use crate::util::SymbolFmt;
use crate::wm::events::{Event, EventId, EventValue};
use crate::wm::WMUtil;

pub struct Mode {
    wrapper: gtk::Box,
    event_id: EventId,
    wm_util: WMUtil,
}

impl Component for Mode {
    fn destroy(&self) {
        self.wm_util.remove_listener(Event::Mode, self.event_id);
        self.wrapper.destroy();
    }
}

impl Mode {
    pub fn init(params: ComponentParams) {
        let ComponentParams { config, window, container, wm_util } = params;
        let label = Label::new(None);
        let wrapper = gtk::Box::new(Orientation::Horizontal, 0);
        super::init_widget(&label, &config, &window, &wrapper);
        container.add(&wrapper);
        wrapper.show();

        let symbols = SymbolFmt::new(config.get_str_or("format", "{mode}"));

        let event_id = wm_util.add_listener(
            Event::Mode,
            clone!(label
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
        ),
        );

        let wm_util = wm_util.clone();
        window.add_component(Box::new(Mode {
            wrapper,
            wm_util,
            event_id,
        }));
    }
}
