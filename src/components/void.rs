use super::{Component, Bar, gtk, ConfigGroup};

pub struct Void {}

impl Component for Void {
    fn init(_container: &gtk::Box, _config: &ConfigGroup, _bar: &Bar) {
    }
}
