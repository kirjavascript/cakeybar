extern crate gio;
extern crate gtk;
extern crate gdk;
extern crate clap;

use gio::prelude::*;
use gtk::prelude::*;

use clap::{Arg, App, SubCommand};

pub static NAME: &str = "cakeybar";

mod util;
mod config;
mod bar;

fn init(application: &gtk::Application, config: &config::Config) {
    for bar_config in config.bars.iter() {
        let _ = bar::Bar::new(&application, bar_config.clone());
    }
    match &config.theme {
        &Some(ref src) => println!("TODO: theme {}", src),
        &None => println!("load default theme"),
    }
}

fn main() {

    // CLI config

    let matches = App::new(NAME)
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

    // Get config

    let config_path = matches.value_of("config")
        .unwrap_or("~/.config/cakeybar/config.toml"); // TODO: xdg

    let config = config::parse_config(config_path);

    // GTK application

    let application = gtk::Application::new(
        &format!("com.kirjava.{}", NAME),
        gio::ApplicationFlags::empty(),
        )
        .expect("Initialization failed...");

    application.connect_startup(move |app| {
        init(&app, &config);
    });
    application.connect_activate(|_| {});

    application.run(&Vec::new()); // dont pass any arguments to GTK

}
