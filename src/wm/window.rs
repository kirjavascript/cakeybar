use crate::wm::ipc::commands::Selectors;
use crate::components::Component;

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
}
