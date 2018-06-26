use super::{Component, Bar, gtk, ComponentConfig};
use gtk::prelude::*;
use gtk::{Label};

use wm::events::{Event, EventValue};

pub struct Window { }

impl Component for Window {
    fn init(container: &gtk::Box, config: &ComponentConfig, bar: &Bar){
        let label = Label::new(None);
        Self::init_widget(&label, config);
        container.add(&label);
        label.show();
        let trunc = config.get_int_or("truncate", 100) as usize;

        bar.wm_util.add_listener(Event::Window, clone!(label
            move |event_opt| {
                if let Some(EventValue::String(name)) = event_opt {
                    let name = if name.len() > trunc {
                        format!("{}...", &name[..trunc])
                    } else {
                        format!("{}", name)
                    };
                    label.set_text(&name);
                }
            }
        ));
    }
}
