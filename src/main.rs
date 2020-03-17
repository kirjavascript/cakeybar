use gio::prelude::*;

#[macro_use]
mod macros;
mod bar;
mod float;
// mod decorations;
mod components;
mod config;
mod util;
mod wm;

pub static NAME: &str = env!("CARGO_PKG_NAME");
pub static VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let args = config::get_args();

    // show monitor debug
    if args.monitors {
        wm::gtk::show_monitor_debug();
        return;
    }

    // send IPC message
    if let Some(message) = args.message {
        wm::ipc::send_message(&message);
        return;
    }

    // GTK application

    // check version
    if let Some(err) = gtk::check_version(3, 22, 0) {
        error!("{} (requires 3.22+)", err);
    }

    let is_multi = args.multi;

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
        let config_path = match &args.config {
            Some(path) => path.to_string(),
            None => format!("{}/config.toml", *config::CONFIG_DIR),
        };

        let config_res = config::parse_file(&config_path);

        if let Ok(config) = config_res {
            // start application
            wm::WMUtil::new(app.clone(), config, &args);
        } else if let Err(msg) = config_res {
            error!("{}", msg);
        }
    });
    application.connect_activate(|_| { });

    application.run(&Vec::new()); // dont pass any arguments to GTK

    if application.get_is_remote() && !is_multi {
        warn!("{} is already running (use -D to force)", NAME);
    }
}
