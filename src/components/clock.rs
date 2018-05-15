use super::{Component, Bar, gtk, ComponentConfig};
use gtk::prelude::*;
use gtk::{Label};

pub struct Clock {
}

impl Component for Clock {
    fn init(container: &gtk::Box, config: &ComponentConfig, bar: &Bar) {
        let label = Label::new(None);
        label.set_text(&"hello world");
        container.add(&label);
    }
}
