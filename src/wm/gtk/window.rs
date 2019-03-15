use glib::translate::ToGlibPtr;
use std::ffi::CString;
use std::os::raw::c_int;

use gtk::{Rectangle, WidgetExt};
use gtk::prelude::*;
use gdk::ScreenExt;

pub fn set_transparent(window: &gtk::Window) {
    set_visual(&window, &None);
    window.connect_screen_changed(set_visual);
    window.connect_draw(draw);
    window.set_app_paintable(true);
}

fn set_visual(window: &gtk::Window, _screen: &Option<gdk::Screen>) {
    if let Some(screen) = window.get_screen() {
        if let Some(visual) = screen.get_rgba_visual() {
            window.set_visual(&visual);
        }
    }
}

fn draw(_window: &gtk::Window, ctx: &cairo::Context) -> Inhibit {
    ctx.set_source_rgba(0., 0., 0., 0.);
    ctx.set_operator(cairo::enums::Operator::Screen);
    ctx.paint();
    Inhibit(false)
}

pub fn keyboard_grab(window: &gtk::Window) -> i32 {
    let ptr: *mut gdk_sys::GdkWindow = window.get_window().unwrap().to_glib_none().0;
    unsafe {
        gdk_sys::gdk_keyboard_grab(ptr, 0, 0)
    }
}

// x11 stuff

pub fn disable_shadow(window: &gtk::Window) {
    let ptr: *mut gdk_sys::GdkWindow = window.get_window().unwrap().to_glib_none().0;

    unsafe {
        let shadow = CString::new("_COMPTON_SHADOW").unwrap();
        let cardinal = CString::new("CARDINAL").unwrap();
        let shadow = gdk_sys::gdk_atom_intern(shadow.as_ptr(), 0);
        let cardinal = gdk_sys::gdk_atom_intern(cardinal.as_ptr(), 0);
        let format: c_int = 32;
        let mode: c_int = 0;
        let data = [0, 0, 0, 0];
        let data_ptr: *const u8 = data.as_ptr();
        let el: c_int = 1;
        gdk_sys::gdk_property_change(
            ptr,      // window:
            shadow,   // property:
            cardinal, // type_:
            format,   // format:
            mode,     // mode:
            data_ptr, // data:
            el,       // nelements:
        );
    }
}

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
        let data = if is_top {
            [
                0, 0, // left
                0, 0, // right
                lo, hi, // top
                0, 0, // bottom
            ]
        } else {
            [
                0, 0, // left
                0, 0, // right
                0, 0, // top
                lo, hi, // bottom
            ]
        };
        let data_ptr: *const u8 = data.as_ptr();
        let el: c_int = 4 as i32;
        gdk_sys::gdk_property_change(
            ptr,      // window:
            strut,    // property:
            cardinal, // type_:
            format,   // format:
            mode,     // mode:
            data_ptr, // data:
            el,       // nelements:
        );
        // partial
        let x1 = rect.x + rect.width - 1;
        let (x0lo, x0hi) = ((rect.x & 0xFF) as u8, (rect.x >> 8) as u8);
        let (x1lo, x1hi) = ((x1 & 0xFF) as u8, (x1 >> 8) as u8);
        let data = if is_top {
            [
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
            ]
        } else {
            [
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
            ]
        };
        // left, right, top, bottom
        let data_ptr: *const u8 = data.as_ptr();
        let el: c_int = 12 as i32;
        gdk_sys::gdk_property_change(
            ptr,      // window:
            partial,  // property:
            cardinal, // type_:
            format,   // format:
            mode,     // mode:
            data_ptr, // data:
            el,       // nelements:
        );
    }
}
