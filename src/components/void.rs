use super::{Component, Bar, gtk, ComponentConfig};

pub struct Void {}

impl Component for Void {
    fn init(_container: &gtk::Box, _config: &ComponentConfig, _bar: &Bar) {}
}
