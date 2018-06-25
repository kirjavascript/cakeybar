use super::{Component, Bar, gtk, ComponentConfig};
use gtk::prelude::*;
use gtk::{Label};
use wm::events::Event;

pub struct I3Mode { }

impl Component for I3Mode {
    fn init(container: &gtk::Box, config: &ComponentConfig, bar: &Bar){
        let label = Label::new(None);
        Self::init_widget(&label, config);
        container.add(&label);

        bar.wm_util.data.borrow_mut()
            .events.add_listener_value(Event::Mode, clone!(label
                move |event_opt: Option<String>| {
                    if let Some(mode) = event_opt {
                        let is_default = mode == "default";

                        if is_default {
                            label.hide();
                        } else {
                            label.show();
                            label.set_text(&mode);
                        }
                    }
                    Ok(())
                }
            )).unwrap();

    }
}
