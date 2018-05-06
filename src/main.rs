// cakeybar
extern crate gio;
extern crate gtk;
extern crate gdk;
extern crate chrono;

use gio::prelude::*;
use gtk::prelude::*;
use gdk::{Screen, ScreenExt};
use gtk::{
    Box,
    Label,
    Orientation,
    Image,
    CssProvider,
    STYLE_PROVIDER_PRIORITY_APPLICATION,
};
use std::env::args;
use std::path::Path;
use chrono::Local;

fn current_time() -> String {
    return format!("{}", Local::now().format("hello world 🍰 %Y-%m-%d %H:%M:%S"));
}



fn build_ui(application: &gtk::Application) {
    let mut window = gtk::Window::new(gtk::WindowType::Toplevel); // use Popup for dmenu
    application.add_window(&window);

    let (height, width) = (Screen::height(), Screen::width());
    let screen = Screen::get_default().unwrap();
    let mut monitors = Vec::new();
    for i in 0..screen.get_n_monitors() {
        monitors.push(screen.get_monitor_geometry(i));
    }
    println!("{:#?}", monitors);

    window.set_title("cakeybar");
    window.set_default_size(0, 27);
    window.set_type_hint(gdk::WindowTypeHint::Dock);
    window.move_(0, height / 2);
    // set screen by manipulating start position


    let container = Box::new(Orientation::Horizontal, 0);

    let img: Image = Image::new_from_file(Path::new("./resources/icon.svg"));
    // img.set_name("icon");
    container.add(&img);

    let time = current_time();
    let label = Label::new(None);
    label.set_text(&time);
    label.set_margin_left(10);
    container.add(&label);

    window.add(&container);
    window.show_all();

    let tick = move || {
        let time = current_time();
        label.set_text(&time);
        gtk::Continue(true)
    };

    gtk::timeout_add_seconds(1, tick);

    // styles

    let style_context = window.get_style_context().unwrap();
    // let style = include_bytes!("../../style/command-input.css");
    let style = r#"
        window {
            padding: 500px;
            background-color: black;
        }
        label {
            color: white;
        }
        #icon {
            margin-bottom: 25px;
        }
    "#;
    let provider = CssProvider::new();
    match provider.load_from_data(style.as_bytes()) {
        Ok(_) => {
            style_context.add_provider(&provider, STYLE_PROVIDER_PRIORITY_APPLICATION);
        },
        Err(_) => println!("Error parsing stylesheet"),
    };
}

fn main() {

    let application = gtk::Application::new(
            "com.kirjava.cakeybar",
            gio::ApplicationFlags::empty(),
        )
        .expect("Initialization failed...");

    application.connect_startup(|app| {
        build_ui(app);
    });
    application.connect_activate(|_| {});

    application.run(&args().collect::<Vec<_>>());

}
