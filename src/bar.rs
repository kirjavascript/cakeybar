use super::{gdk, gtk};
use gio::prelude::*;
use gtk::prelude::*;
use gtk::{
    Window,
    WindowType,

    // Box,
    // Label,
    // WidgetExt,
    // Orientation,
    // Image,
    // CssProvider,
    // STYLE_PROVIDER_PRIORITY_APPLICATION,
};


pub struct Bar {
    app: &gtk::Application,
}

impl Bar {
    pub fn new(application: &gtk::Application) -> Bar {

        let mut window = Window::new(WindowType::Toplevel);
        application.add_window(&window);

        window.set_title("cakeybar");
        window.set_default_size(0, 27);
        window.set_type_hint(gdk::WindowTypeHint::Dock);
        // window.move_(0, height / 2);
        // set screen by manipulating start position

        window.show_all();

        Bar { }
    }
}
