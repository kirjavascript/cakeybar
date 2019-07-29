// account for border width
// TODO: only show visible

// iterate and == diff on the decoration side

// XWindows struct .update()

use std::collections::HashMap;

use gtk::{Window, WindowType, WidgetExt};
use gdk::WindowExt;

use crate::wm;
use crate::wm::events::{Event, EventId, EventValue};
use crate::wm::xcb::windows::XWindowData;

use gtk::prelude::*;

pub fn load_decorations(wm_util: &wm::WMUtil) {

    let mut windows: HashMap<xcb::Window, XWindowData> = HashMap::new();
    let mut gtk_windows: HashMap<xcb::Window, gtk::Window> = HashMap::new();

    let window = gtk::Window::new(WindowType::Popup);
    wm_util.add_gtk_window(&window);

        let label = gtk::Label::new(None);

        label.set_text("custom window decoration WIP");
        label.show();

        let container = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        container.add(&label);
        window.add(&container);
        WidgetExt::set_name(&container, "info");

    WidgetExt::set_name(&window, "info");

    // window.set_type_hint(gdk::WindowTypeHint::Dock);
    window.set_title("decorations");
    // window.set_keep_below(true);

    // window.set_skip_pager_hint(false);
    // window.set_skip_taskbar_hint(false);
    // window.set_o
    window.set_decorated(false);
    window.move_(100, 900);
    window.resize(700, 500);
    // window.stick();

    wm::gtk::set_transparent(&window);
    window.show_all();

    if let Some(window) = window.get_window() {
        window.set_override_redirect(true);
        // window.set_pass_through(true);
        // window.set_static_gravities(true);
        // window.set_keep_below(true);
        // window.set_modal_hint(true);
        window.set_shadow_width(0, 0, 0, 0);
        // window.show();
        // window.show_unraised();


        extern "C" {
            pub fn gdk_x11_window_get_xid(window: gdk::Window) -> u32;
        }
        let id = unsafe {
            let id = gdk_x11_window_get_xid(window.clone());
            // println!("{:#?}", id);
            id
        };


        // wait a tick before restacking window

        gtk::idle_add(move || {
            match xcb::Connection::connect(None) {
                Ok((conn, screen_num)) => {
                    let screen = conn.get_setup().roots().nth(screen_num as usize).unwrap();
                    // let parent = xcb::query_tree(&conn, id).get_reply().unwrap().parent();
                    // xcb::configure_window(
                    //     &conn,
                    //     parent,
                    //     &[(xcb::CONFIG_WINDOW_STACK_MODE as u16, xcb::STACK_MODE_BELOW)],
                    // );
                    xcb::configure_window(
                        &conn,
                        id,
                        &[(xcb::CONFIG_WINDOW_STACK_MODE as u16, xcb::STACK_MODE_BELOW)],
                    );
                    conn.flush();
                }
                _ => {},
            }
            gtk::Continue(false)
        });
    }

// 19:48 <+bdelloid> it's plausible gtk is using multiple virtual windows to draw their shit
// 19:48 <+bdelloid> I think I had that issue before
// 19:48 <+bdelloid> you need to find the real windos
// 19:48 <+bdelloid> window
// 19:48 <+bdelloid> up the chain

    let event_id = wm_util.add_listener(Event::Windows,
        clone!(window move |windows_opt| {
            if let Some(EventValue::Windows(event_windows)) = windows_opt {
                let vim = 6291459;
                println!("{:#?}", event_windows);
                if let Some(v) = event_windows.get(&vim) {
                    window.move_(v.0 as i32 - 20 , v.1 as i32 - 20);
                }

                // set behind
                if let Some(window) = window.get_window() {


                            // xcb::circulate_window(
                            //     &conn,
                            //     1,
                            //     id,
                            // );



        extern "C" {
            pub fn gdk_x11_window_get_xid(window: gdk::Window) -> u32;
        }
        let id = unsafe {
            let id = gdk_x11_window_get_xid(window.clone());
            // println!("{:#?}", id);
            id
        };

        match xcb::Connection::connect(None) {
            Ok((conn, screen_num)) => {


                let screen = conn.get_setup().roots().nth(screen_num as usize).unwrap();

                // xcb::unmap_window(&conn, vim);
                // conn.flush();


                // xcb::change_property(
                xcb::configure_window(

                    &conn,
                    id,
                    &[
                    // (xcb::CONFIG_WINDOW_SIBLING as u16, screen.root()),
                    (xcb::CONFIG_WINDOW_STACK_MODE as u16, xcb::STACK_MODE_BELOW),
                    ],
                );

                // xcb::configure_window(

                //     &conn,
                //     vim,
                //     &[
                //     (xcb::CONFIG_WINDOW_SIBLING as u16, id),
                //     (xcb::CONFIG_WINDOW_STACK_MODE as u16, xcb::STACK_MODE_ABOVE),
                //     ],
                // );
                println!("{:#?}", id);

                            // xcb::circulate_window(
                            //     &conn,
                            //     1,
                            //     vim,
                            // );

                // xcb::change_property(
                //     &conn,
                //     xcb::PROP_MODE_REPLACE as u8,
                //     vim,
                //     xcb::ATOM_WM_TRANSIENT_FOR,
                //     xcb::ATOM_WINDOW,
                //     32,
                //     &[id],
                // );

                            // xcb::circulate_window(
                            //     &conn,
                            //     1,
                            //     id,
                            // );

                // xcb::map_window(&conn, vim);
                conn.flush();
            }
            _ => {},
        }

                }
            }
        })
    );


}
