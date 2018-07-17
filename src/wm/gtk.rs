use {gtk, gdk_sys};
use gtk::{
    Rectangle,
    WidgetExt,
    CssProvider,
    CssProviderExt,
    StyleContext,
};
use gdk::{
    Screen,
    Display,
    DisplayExt,
    MonitorExt,
};

pub fn load_theme(path: &str) {
    let screen = Screen::get_default().unwrap();
    let provider = CssProvider::new();
    match provider.load_from_path(path) {
        Ok(_) => StyleContext::add_provider_for_screen(&screen, &provider, 0),
        Err(e) => {error!("{}", e);},
    };
}

// monitor stuff

pub fn get_monitor_geometry() -> Vec<Rectangle> {
    let display = Display::get_default().unwrap();
    let mut monitors = Vec::new();
    for i in 0..display.get_n_monitors() {
        let monitor = display.get_monitor(i).unwrap();
        monitors.push(monitor.get_geometry())
    }
    monitors
}

pub fn get_monitor_name(monitor_index: i32) -> Option<String> {
    let display = Display::get_default()?;
    let monitor = display.get_monitor(monitor_index)?;
    monitor.get_model()
}

pub fn show_monitor_debug() {
    gtk::init().ok();
    let display = Display::get_default().unwrap();
    for i in 0..display.get_n_monitors() {
        if let Some(monitor) = display.get_monitor(i) {
            let geometry = monitor.get_geometry();
            let Rectangle { x, y, width, height } = geometry;
            let model = monitor.get_model().unwrap_or("".to_string());
            println!("Monitor {}: {} @ {}x{} x: {} y: {}",
                i, model, width, height, x, y);
        }
    }
}

// x11 stuff

use glib::translate::ToGlibPtr;
use std::ffi::CString;
use std::os::raw::c_int;

// seems ignored in bspwm, i3 and awesome. why did I write this?
pub fn set_strut(window: &gtk::Window, is_top: bool, rect: Rectangle) {
    let ptr: *mut gdk_sys::GdkWindow = window.get_window().unwrap().to_glib_none().0;

    unsafe {
        // atoms
        let strut = CString::new("_NET_WM_STRUT").unwrap();
        let partial = CString::new("_NET_WM_STRUT_PARTIAL").unwrap();
        let cardinal = CString::new("CARDINAL").unwrap();
        let strut = gdk_sys::gdk_atom_intern(strut.as_ptr(), 0);
        let cardinal = gdk_sys::gdk_atom_intern(cardinal.as_ptr(), 0);
        let partial = gdk_sys::gdk_atom_intern(partial.as_ptr(), 0);
        // strut
        let format: c_int = 16; // number of bits (must be 8, 16 or 32)
        let mode: c_int = 0; // PROP_MODE_REPLACE
        // get height bytes
        let (lo, hi) = ((rect.height & 0xFF) as u8, (rect.height >> 8) as u8);
        let data = if is_top {[
            0, 0, // left
            0, 0, // right
            lo, hi, // top
            0, 0, // bottom
        ]} else {[
            0, 0, // left
            0, 0, // right
            0, 0, // top
            lo, hi, // bottom
        ]};
        let data_ptr: *const u8 = data.as_ptr();
        let el: c_int = 4 as i32;
        gdk_sys::gdk_property_change(
            ptr, // window:
            strut, // property:
            cardinal, // type_:
            format, // format:
            mode, // mode:
            data_ptr, // data:
            el, // nelements:
        );
        // partial
        let x1 = rect.x + rect.width - 1;
        let (x0lo, x0hi) = ((rect.x & 0xFF) as u8, (rect.x >> 8) as u8);
        let (x1lo, x1hi) = ((x1 & 0xFF) as u8, (x1 >> 8) as u8);
        let data = if is_top {[
            0, 0, // left
            0, 0, // right
            lo, hi, // top
            0, 0, // bottom
            0, 0, // start
            0, 0, // end
            0, 0, // start
            0, 0, // end
            x0lo, x0hi, // start
            x1lo, x1hi, // end
            0, 0, // start
            0, 0, // end
        ]} else {[
            0, 0, // left
            0, 0, // right
            0, 0, // top
            lo, hi, // bottom
            0, 0, // start
            0, 0, // end
            0, 0, // start
            0, 0, // end
            0, 0, // start
            0, 0, // end
            x0lo, x0hi, // start
            x1lo, x1hi, // end
        ]};
        // left, right, top, bottom
        let data_ptr: *const u8 = data.as_ptr();
        let el: c_int = 12 as i32;
        gdk_sys::gdk_property_change(
            ptr, // window:
            partial, // property:
            cardinal, // type_:
            format, // format:
            mode, // mode:
            data_ptr, // data:
            el, // nelements:
        );
    }
}
