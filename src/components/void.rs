use super::{Component, Bar, gtk, ComponentConfig};

pub struct Void {}

impl Component for Void {
    fn init(container: &gtk::Box, config: &ComponentConfig, bar: &Bar) {}
}
