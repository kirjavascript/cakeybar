mod message;

use super::{Component, Bar, gtk, ComponentConfig};
use gtk::prelude::*;
use gtk::{Orientation};
use gdk::{WindowExt, RGBA};
use std;
use glib;
use crossbeam_channel as channel;
use std::thread;
use self::message::Message;

pub struct Tray { }

// mutable statics should be safe within the same thread
static mut TRAY_LOADED: bool = false;

impl Component for Tray {
    fn init(container: &gtk::Box, config: &ComponentConfig, bar: &Bar) {
        if unsafe { !TRAY_LOADED } {
            unsafe { TRAY_LOADED = true; }
            Tray::be_a_tray(container, config, bar);
        }
        else {
            warn!("tray component is already loaded");
        }
    }
}

impl Tray {

    fn be_a_tray(container: &gtk::Box, config: &ComponentConfig, bar: &Bar) {
        // extra surrounding base widget added for margins, etc
        let wrapper = gtk::Box::new(Orientation::Horizontal, 0);
        let base_widget = gtk::Box::new(Orientation::Horizontal, 0);
        base_widget.add(&wrapper);
        base_widget.show_all();
        Self::init_widget(&base_widget, container, config, bar);

        let (s_main, r_main) = channel::unbounded();

        s_main.send(1);

        thread::spawn(clone!(s_main move || {
            // capture signals
            glib::source::unix_signal_add(2, clone!(s_main move || {
                s_main.send(2);
                gtk::Continue(false)
            })); // SIGINT
            glib::source::unix_signal_add(15, move || {
                s_main.send(15);
                gtk::Continue(false)
            }); // SIGTERM

            loop {
                if let Some(err) = r_main.recv() {
                    println!("{:#?}", err);
                    if err == 2 || err == 15 {
                        error!("received kill signal");
                        std::process::exit(0);
                    }
                }
            }

        }));

    }

    // #[deprecated]
    // fn _load(container: &gtk::Box, config: &ComponentConfig, bar: &Bar) {
    //     // extra surrounding base widget added for margins, etc
    //     let wrapper = gtk::Box::new(Orientation::Horizontal, 0);
    //     let base_widget = gtk::Box::new(Orientation::Horizontal, 0);
    //     base_widget.add(&wrapper);
    //     base_widget.show_all();
    //     Self::init_widget(&base_widget, container, config, bar);

    //     // init
    //     let (tx_ipc, rx_ipc) = ::tray::ipc::get_server();
    //     ::tray::as_subprocess();

    //     // get bg color
    //     if let Some(ctx) = base_widget.get_style_context() {
    //         #[allow(deprecated)] // ctx.get_property doesn't work
    //         let RGBA { red, green, blue, .. } = ctx.get_background_color(gtk::StateFlags::NORMAL);
    //         let bg_color = (((red * 255.) as u32) << 16) + (((green * 255.) as u32) << 8) + (blue * 255.) as u32;
    //         tx_ipc.send(Message::BgColor(bg_color));
    //     }

    //     // set icon size
    //     let icon_size = config.get_int_or("icon-size", 20);
    //     if icon_size != 20 {
    //         tx_ipc.send(Message::IconSize(icon_size as u16));
    //     }

    //     // send resize event
    //     wrapper.connect_size_allocate(move |c, rect| {
    //         let w = c.get_window().unwrap();
    //         let (_zo, xo, yo) = w.get_origin();
    //         let y = (yo + (rect.y + ((rect.height - (icon_size as i32))/2))) as u32;
    //         let x = (xo + rect.x) as u32;
    //         tx_ipc.send(Message::Move(x, y));
    //     });

    //     // receive events
    //     gtk::timeout_add(10, clone!((base_widget, wrapper) move || {
    //         if let Ok(msg) = rx_ipc.try_recv() {
    //             match msg {
    //                 Message::Width(w) => {
    //                     wrapper.set_size_request(w as i32, icon_size as i32);
    //                     // the next lines fix a background display bug
    //                     base_widget.hide();
    //                     base_widget.show();
    //                 },
    //                 _ => {},
    //             }
    //         }
    //         gtk::Continue(true)
    //     }));
    // }
}
