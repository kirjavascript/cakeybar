use super::{Component, Bar, gtk, ComponentConfig};
use gtk::prelude::*;
use gtk::{Label};
use wm::events::{Event, EventValue};

pub struct Mode { }

impl Component for Mode {
    fn init(container: &gtk::Box, config: &ComponentConfig, bar: &Bar){
        let label = Label::new(None);
        Self::init_widget(&label, config);
        container.add(&label);

        bar.wm_util.add_listener(Event::Mode, clone!(label
            move |event_opt| {
                if let Some(EventValue::String(mode)) = event_opt {
                    let is_default = mode == "default";

                    if is_default {
                        label.hide();
                    } else {
                        label.show();
                        label.set_text(&mode);
                    }
                }
            }
        ));
    }
}
