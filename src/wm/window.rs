use crate::wm;
use crate::wm::ipc::commands::Selectors;
use crate::components::Component;
use crate::config::ConfigGroup;

pub trait Window {
    fn destroy(&self);
    fn relayout(&self);
    fn show(&self);
    fn hide(&self);
    fn to_window(&self) -> gtk::Window;
    fn get_container(&self) -> &gtk::Box;
    fn get_overlay(&self) -> &gtk::Overlay;
    fn add_component(&mut self, _: Box<dyn Component>);
    fn matches_selectors(&self, _: &Selectors) -> bool;
    fn load_component(&mut self, config: ConfigGroup, container: &gtk::Box, wm_util: &wm::WMUtil);
}
