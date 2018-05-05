extern crate gio;
extern crate gtk;
extern crate gdk;
extern crate chrono;

use gio::prelude::*;
use gtk::prelude::*;
use std::env::args;
use chrono::Local;

fn current_time() -> String {
    return format!("{}", Local::now().format("%Y-%m-%d %H:%M:%S"));
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::Window::new(gtk::WindowType::Toplevel); // Popup
    application.add_window(&window);

    window.set_title("rustybar");
    window.set_border_width(10);
    window.set_default_size(1080, 20);
    window.set_type_hint(gdk::WindowTypeHint::Dock);
    window.move_(0, 1000000);

    // set screen

    let time = current_time();
    let label = gtk::Label::new(None);
    label.set_text(&time);

    window.add(&label);

    // window.show_all();
    window.show_all();

    let tick = move || {
        let time = current_time();
        label.set_text(&time);
        gtk::Continue(true)
    };

    gtk::timeout_add_seconds(1, tick);
}

fn main() {

    let application = gtk::Application::new(
            "com.kirjava.rustybar",
            gio::ApplicationFlags::empty(),
        )
        .expect("Initialization failed...");

    application.connect_startup(|app| {
        build_ui(app);
    });
    application.connect_activate(|_| {});

    application.run(&args().collect::<Vec<_>>());

}
