use crate::wm::ipc::commands::Selectors;

// enum WindowType {
//     Bar,
//     Float,
// }


pub trait Window {
    fn destroy(&self);
    fn relayout(&self);
    fn show(&self);
    fn hide(&self);
    fn to_window(&self) -> gtk::Window;
    fn matches_selectors(&self, _: &Selectors) -> bool;
}
