use super::{Component, Bar, gtk, ComponentConfig};
use gtk::prelude::*;
use gtk::{Label};

use wm::events::Event;

pub struct Window { }

impl Component for Window {
    fn init(container: &gtk::Box, config: &ComponentConfig, bar: &Bar){
        let label = Label::new(None);
        Self::init_widget(&label, config);
        container.add(&label);
        label.show();
        let trunc = config.get_int_or("truncate", 100) as usize;

        // bar.wm_util.data.borrow_mut()
        //     .events.add_listener_value(Event::Window, clone!(label
        //         move |event_opt: Option<String>| {
        //             if let Some(name) = event_opt {
        //                 let name = if name.len() > trunc {
        //                     format!("{}...", &name[..trunc])
        //                 } else {
        //                     format!("{}", name)
        //                 };
        //                 label.set_text(&name);
        //             }
        //             Ok(())
        //         }
        //     )).unwrap();
    }
}
