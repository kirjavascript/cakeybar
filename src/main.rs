use clap::{App, Arg};
use gio::prelude::*;

#[macro_use]
mod macros;
mod bar;
mod components;
mod config;
mod util;
mod wm;

pub static NAME: &str = env!("CARGO_PKG_NAME");
pub static VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    // CLI config

    let matches = App::new(NAME)
        .version(VERSION)
        .setting(if *config::NO_COLOR {
            clap::AppSettings::ColorNever
        } else {
            clap::AppSettings::ColorAuto
        })
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("Specify a config path")
            .takes_value(true))
        .arg(Arg::with_name("watch")
            .short("w")
            .long("watch")
            .help("Watch config files and reload on changes"))
        .arg(Arg::with_name("message")
            .short("m")
            .long("message")
            .value_name("MESSAGE")
            .help("Send an IPC message")
            .takes_value(true))
        .arg(Arg::with_name("monitors")
            .short("M")
            .long("monitors")
            .help("Shows information about monitors"))
        .arg(Arg::with_name("multi")
            .short("D")
            .long("multi")
            .help("Allow multiple instances")
            .hidden(true))
        .get_matches();

    // show monitor debug
    if matches.is_present("monitors") {
        wm::gtk::show_monitor_debug();
        return;
    }

    // send IPC message
    if let Some(message) = matches.value_of("message") {
        wm::ipc::send_message(message);
        return;
    }

    // GTK application

    // check version
    if let Some(err) = gtk::check_version(3, 22, 0) {
        error!("{} (requires 3.22+)", err);
    }

    let is_multi = matches.is_present("multi");

    let application = gtk::Application::new(
        format!("com.kirjava.{}", NAME).as_str(),
        if is_multi {
            gio::ApplicationFlags::NON_UNIQUE
        } else {
            gio::ApplicationFlags::empty()
        },
    ).expect("initialization failed...");


    application.connect_startup(move |app| {
        // get config path
        let config_path = match matches.value_of("config") {
            Some(path) => path.to_string(),
            None => format!("{}/config.toml", *config::CONFIG_DIR),
        };

        let config_res = config::parse_config(&config_path);

        if let Ok(config) = config_res {
            // start application
            wm::WMUtil::new(app.clone(), config, &matches);
        } else if let Err(msg) = config_res {
            error!("{}", msg);
        }
    });
    application.connect_activate(|_| { });

    application.run(&Vec::new()); // dont pass any arguments to GTK

    if application.get_is_remote() && !is_multi {
        warn!("{} is already running (use -D to force)", NAME);
        wm::ipc::send_message("reload config");
    }

}
