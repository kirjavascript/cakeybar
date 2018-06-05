extern crate gio;
extern crate gtk;
extern crate gdk;
extern crate glib;
extern crate clap;
extern crate toml;
// used in components
extern crate i3ipc;
extern crate chrono;
extern crate systemstat;
extern crate probes;
extern crate sysinfo;
// used in tray
// #[macro_use]
// extern crate chan;
// extern crate chan_signal;
// extern crate xcb;

use gio::prelude::*;

use clap::{Arg, App, SubCommand};

#[macro_use]
mod macros;
mod util;
mod config;
mod bar;
mod components;
// mod tray;

pub static NAME: &str = "cakeybar";

fn init(application: &gtk::Application, config: &config::Config) {
    // load theme to screen
    match &config.theme {
        &Some(ref src) => util::load_theme(src),
        &None => {/* default theme */},
    }
    // load bars
    for bar_config in config.bars.iter() {
        let _ = bar::Bar::new(
            &application,
            &bar_config,
            &config.components,
        );
    }

    // attempt to stack gtk bar BELOW tray
    // if let Ok((conn, preferred)) = xcb::Connection::connect(None) {
    //     let setup = conn.get_setup();
    //     let screen = setup.roots().nth(preferred as usize).unwrap();
    //     let value_mask = (xcb::CONFIG_WINDOW_STACK_MODE | xcb::CONFIG_WINDOW_SIBLING) as u16;

    //     if let Ok(reply) = xcb::query_tree(&conn, screen.root()).get_reply() {
    //         let i3_opt = reply.children().iter().find(|child| {
    //              tray::tray::xcb_get_wm_name(&conn, **child).contains("i3")
    //         });
    //         let bar_opt = reply.children().iter().find(|child| {
    //             NAME == tray::tray::xcb_get_wm_name(&conn, **child)
    //         });
    //         if let Some(bar) = bar_opt {
    //             if let Some(i3) = i3_opt {
    //                 println!("{:#?}", (i3, bar));

    //                 xcb::configure_window_checked(&conn, *i3, &[
    //                     (value_mask, *bar),
    //                     (value_mask, xcb::STACK_MODE_ABOVE),
    //                 ]);

    //                 println!("{:#?}", "swapped");
    //                 conn.flush();
    //             }
    //         }
    //     }

    //     if let Ok(reply) = xcb::query_tree(&conn, screen.root()).get_reply() {
    //         for i in reply.children() {
    //             println!("{:#?} {}", tray::tray::xcb_get_wm_name(&conn, *i), i);
    //         }
    //     }
    // }
}

fn main() {

    // CLI config

    let matches = App::new(NAME)
        .arg(Arg::with_name("config")
             .short("c")
             .long("config")
             .value_name("FILE")
             .help("Specify a config path")
             .takes_value(true))
        .subcommand(SubCommand::with_name("monitors")
            .about("Shows information about monitors")
            .arg(Arg::with_name("monitors").short("m")))
        .get_matches();

    // show monitor debug
    if matches.is_present("monitors") {
        util::show_monitor_debug();
        return ();
    }

    // get config

    let default_path = format!("~/.config/{}/config.toml", NAME); // TODO: xdg
    let config_path = matches.value_of("config").unwrap_or(&default_path);

    let config = config::parse_config(config_path);

    // load tray
    // tray::init();

    // GTK application

    // check version
    if let Some(err) = gtk::check_version(3, 22, 0) {
        eprintln!("{} (requires 3.22+)", err);
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
