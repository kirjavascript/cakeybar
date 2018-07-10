extern crate cairo;
extern crate gio;
extern crate gtk;
extern crate gdk;
extern crate gdk_sys;
extern crate glib;
extern crate xcb;
extern crate clap;
extern crate toml;
extern crate ansi_term;
// components
extern crate i3ipc;
extern crate chrono;
extern crate systemstat;
extern crate probes;
extern crate sysinfo;
// tray
#[macro_use]
extern crate chan;
extern crate chan_signal;
// tray ipc
extern crate bincode;
#[macro_use]
extern crate serde_derive;
extern crate serde;

use gio::prelude::*;

use clap::{Arg, App};

#[macro_use]
mod macros;
mod util;
mod config;
mod bar;
mod components;
mod tray;
mod wm;

pub static NAME: &str = env!("CARGO_PKG_NAME");
pub static VERSION: &str = env!("CARGO_PKG_VERSION");

fn init(application: &gtk::Application, config: &config::Config) {
    // load theme to screen
    match &config.theme {
        &Some(ref src) => wm::gtk::load_theme(src),
        &None => {/* default theme */},
    }
    let wm_util = wm::WMUtil::new();
    // load bars
    for bar_config in config.bars.iter() {
        let _ = bar::Bar::new(
            &application,
            &bar_config,
            &config.components,
            &wm_util,
        );
    }
}

fn main() {

    // CLI config

    let matches = App::new(NAME)
        .version(VERSION)
        .arg(Arg::with_name("config")
             .short("c")
             .long("config")
             .value_name("FILE")
             .help("Specify a config path")
             .takes_value(true))
        .arg(Arg::with_name("monitors")
             .short("m")
             .long("monitors")
             .help("Shows information about monitors"))
        .arg(Arg::with_name("tray")
             .short("t")
             .long("tray")
             .help("Loads system tray")
             .hidden(true))
        .get_matches();

    // show monitor debug
    if matches.is_present("monitors") {
        wm::gtk::show_monitor_debug();
        return ();
    }
    // load tray
    else if matches.is_present("tray") {
        tray::main();
        return ();
    }

    // get config

    let default_path = format!("{}/config.toml", util::get_config_dir());

    let config_path = matches.value_of("config").unwrap_or(&default_path);

    let config = config::parse_config(config_path);

    // GTK application

    // check version
    if let Some(err) = gtk::check_version(3, 22, 0) {
        warn!("{} (requires 3.22+)", err);
    }

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
