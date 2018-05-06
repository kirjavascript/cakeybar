extern crate gio;
extern crate gtk;
extern crate gdk;
extern crate clap;

use gio::prelude::*;
use gtk::prelude::*;

use clap::{Arg, App, SubCommand};

mod util;

fn init(application: &gtk::Application) {

    let monitors = util::get_monitors();

    println!("{:#?}", monitors);

    // get config
}

fn main() {

    // CLI config

    let matches = App::new("cakeybar")
        .arg(Arg::with_name("config")
             .short("c")
             .long("config")
             .value_name("FILE")
             .help("Sets a custom config file")
             .takes_value(true))
        .subcommand(SubCommand::with_name("monitors")
            .about("Shows information about monitors")
            .arg(Arg::with_name("monitors").short("m")))
        .get_matches();

    // show monitor debug
    if matches.is_present("monitors") {
        gtk::init();
        let (width, height) = util::get_dimensions();
        println!("Screen: {}x{}", width, height);
        let monitors = util::get_monitors();
        for (i, mon) in monitors.iter().enumerate() {
            let &gdk::Rectangle { x, y, width, height } = mon;
            println!("Monitor {}: {}x{} x: {} y: {}", i, width, height, x, y);
        }
        return ();
    }

    // TODO: xdg

    let config = matches.value_of("config")
        .unwrap_or("~/.config/cakeybar/config.toml");
    println!("Value for config: {}", config);


    // GTK application

    let application = gtk::Application::new(
        "com.kirjava.cakeybar",
        gio::ApplicationFlags::empty(),
        )
        .expect("Initialization failed...");

    application.connect_startup(|app| {
        init(app);
    });
    application.connect_activate(|_| {});

    application.run(&Vec::new());

}
