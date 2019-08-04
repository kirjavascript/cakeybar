use crate::components::{Component, ComponentParams};
use glib::markup_escape_text;
use gtk;
use gtk::prelude::*;
use gtk::Label;
use crate::util::SymbolFmt;

use crate::wm::events::{Event, EventId, EventValue};
use crate::wm::WMUtil;

pub struct WindowTitle {
    label: Label,
    event_id: EventId,
    wm_util: WMUtil,
}

impl Component for WindowTitle {
    fn destroy(&self) {
        self.wm_util.remove_listener(Event::WindowTitle, self.event_id);
        self.label.destroy();
    }
}

impl WindowTitle {
    pub fn init(params: ComponentParams) {
        let ComponentParams { config, window, wm_util, container } = params;
        let label = Label::new(None);
        super::init_widget(&label, &config, &window, container);
        label.show();

        let trunc = config.get_int_or("truncate", 100) as usize;
        let symbols = SymbolFmt::new(config.get_str_or("format", "{title}"));

        let event_id = wm_util.add_listener(Event::WindowTitle, clone!(label
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

        let wm_util = wm_util.clone();
        window.add_component(Box::new(WindowTitle {
            label,
            wm_util,
            event_id,
        }));
    }
}
