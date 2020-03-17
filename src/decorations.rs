// account for border width
// iterate and == diff on the decoration side

// XWindows struct .update()

// https://stackoverflow.com/questions/31225743/x11-non-clipped-child-windows/31436071#31436071
// https://turbomack.github.io/posts/2019-07-28-rust-vs-gui.html
// https://jichu4n.com/posts/how-x-window-managers-work-and-how-to-write-one-part-iii/
// https://stackoverflow.com/questions/37502000/how-to-draw-titlebar-with-xcb
// https://github.com/minos-org/i3/blob/36fc8fcab311dc7343f6574ea7ea6eb56f87feb0/src/manage.c#L81

use std::collections::HashMap;

use gtk::{Window, WindowType, WidgetExt};
use gdk::WindowExt;

use std::cell::RefCell;
use std::rc::Rc;

use crate::wm;
use crate::wm::events::{Event, EventId, EventValue};
use crate::wm::xcb::xwindows::XWindowData;
use crate::wm::gtk::gdk_get_xid;

use gtk::prelude::*;

pub fn load_decorations(wm_util: &wm::WMUtil) {

    match xcb::Connection::connect(None) {
        Ok((conn, screen_num)) => {
            let vim: xcb::Window = 50331651;

        },
        _ => {},
    }
}

pub fn _load_decorations(wm_util: &wm::WMUtil) {

    let mut gtk_windows: Rc<RefCell<HashMap<xcb::Window, gtk::Window>>> =
        Rc::new(RefCell::new(HashMap::new()));

    let event_id = wm_util.add_listener(Event::Windows,
        clone!((gtk_windows, wm_util) move |windows_opt| {
            if let Some(EventValue::Windows(event_windows)) = windows_opt {
                for (xwindow, xwindowdata) in event_windows {
                    let XWindowData { x, y, width, height, name, visible } = xwindowdata;
                    let has_existing = {
                        if let Some(window) = gtk_windows.borrow().get(&xwindow) {

                            window.set_title(&name);
                            // window.move_(x as i32 + 20, y as i32 + 20);
                            // window.resize(width as i32 - 40, height as i32 - 40);
                            // window.resize(40,40);

                            // if visible {
                            //     window.show();
                            // } else {
                            //     window.hide();
                            // }

                            // if let Some(window) = window.get_window() {
                            //     let id = gdk_get_xid(&window);

                            //     match xcb::Connection::connect(None) {
                            //         Ok((conn, screen_num)) => {
                            //             let screen = conn.get_setup().roots().nth(screen_num as usize).unwrap();
                            //             xcb::configure_window(
                            //                 &conn,
                            //                 id,
                            //                 &[(xcb::CONFIG_WINDOW_STACK_MODE as u16, xcb::STACK_MODE_BELOW)],
                            //             );
                            //             conn.flush();
                            //         }
                            //         _ => {},
                            //     }
                            // }

                            true
                        } else {
                            false
                        }
                    };
                    if !has_existing {
                        // new window
                        let window = gtk::Window::new(WindowType::Popup);
                        wm_util.add_gtk_window(&window);

                        if let Some(window) = window.get_window() {
                            window.set_override_redirect(true);
                            window.set_shadow_width(0, 0, 0, 0);

                            let id = gdk_get_xid(&window);

                            match xcb::Connection::connect(None) {
                                Ok((conn, screen_num)) => {
                                    // let screen = conn.get_setup().roots().nth(screen_num as usize).unwrap();
                                    xcb::configure_window(
                                        &conn,
                                        id,
                                        &[(xcb::CONFIG_WINDOW_STACK_MODE as u16, xcb::STACK_MODE_BELOW)],
                                    );
                                    conn.flush();
                                    // xcb::unmap_window(&conn, id);
                                    // xcb::reparent_window(
                                    //     &conn,
                                    //     id,
                                    //     xwindow,
                                    //     10,
                                    //     10,
                                    // );
                                    // xcb::map_window(&conn, id);
                                    // conn.flush();
                                }
                                _ => {},
                            }
                        }

        let label = gtk::Label::new(None);

        label.set_text(&name);
        label.show();

        let container = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        container.add(&label);
        window.add(&container);
        WidgetExt::set_name(&container, "info");

                        WidgetExt::set_name(&window, "info");

                        window.set_decorated(false);
                        wm::gtk::set_transparent(&window);
                        window.show_all();

                        window.set_title(&name);
                        // window.move_(x as i32 + 20, y as i32 + 20);
                        // window.resize(width as i32 -40, height as i32 - 40);
                        window.resize(50, 50);

                        if !visible {
                            window.hide();
                        }

                        gtk_windows.borrow_mut().insert(xwindow, window);
                    }
                }

                // remove windows

            }
        })
    );

    //             // xcb::unmap_window(&conn, vim);
    //             // conn.flush();


    //             xcb::configure_window(
    //                 &conn,
    //                 id,
    //                 &[
    //                 // (xcb::CONFIG_WINDOW_SIBLING as u16, screen.root()),
    //                 (xcb::CONFIG_WINDOW_STACK_MODE as u16, xcb::STACK_MODE_BELOW),
    //                 ],
    //             );

    //             // xcb::configure_window(

    //             //     &conn,
    //             //     vim,
    //             //     &[
    //             //     (xcb::CONFIG_WINDOW_SIBLING as u16, id),
    //             //     (xcb::CONFIG_WINDOW_STACK_MODE as u16, xcb::STACK_MODE_ABOVE),
    //             //     ],
    //             // );
    //             // println!("{:#?}", id);

    //                         // xcb::circulate_window(
    //                         //     &conn,
    //                         //     1,
    //                         //     vim,
    //                         // );

    //             // xcb::change_property(
    //             //     &conn,
    //             //     xcb::PROP_MODE_REPLACE as u8,
    //             //     vim,
    //             //     xcb::ATOM_WM_TRANSIENT_FOR,
    //             //     xcb::ATOM_WINDOW,
    //             //     32,
    //             //     &[id],
    //             // );

    //                         // xcb::circulate_window(
    //                         //     &conn,
    //                         //     1,
    //                         //     id,
    //                         // );


}
