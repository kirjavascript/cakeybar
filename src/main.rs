extern crate gio;
extern crate gtk;
extern crate chrono;

use gio::prelude::*;
use gtk::prelude::*;
use std::env::args;
use chrono::Local;

fn current_time() -> String {
    return format!("{}", Local::now().format("%Y-%m-%d %H:%M:%S"));
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("rustybar");
    window.set_border_width(10);
    window.set_default_size(260, 40);
    window.set_modal(true);

    let time = current_time();
    let label = gtk::Label::new(None);
    label.set_text(&time);

    window.add(&label);

    window.show_all();
    window.move_(0, 0);
    window.stick();
    window.set_accept_focus(false);
    window.set_decorated(false);

    let tick = move || {
        let time = current_time();
        label.set_text(&time);
        gtk::Continue(true)
    };

    gtk::timeout_add_seconds(1, tick);

    window.connect_property_window_position_notify(move |_| {
        println!("hello");
    });
}

fn main() {
    let mut flags = gio::ApplicationFlags::empty();
    flags.insert(gio::ApplicationFlags::IS_SERVICE);
    let application = gtk::Application::new(
            "com.kirjava.rustybar",
            flags,
        )
        .expect("Initialization failed...");

    application.connect_startup(|app| {
        build_ui(app);
    });
    application.connect_activate(|_| {});

    application.run(&args().collect::<Vec<_>>());
}
