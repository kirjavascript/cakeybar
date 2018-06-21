use {gdk, gdk_sys, gtk};
use gtk::prelude::*;
use gtk::{
    Window,
    WindowType,
    Orientation,
    Rectangle,
};
use gdk::ScrollDirection;
use glib::translate::ToGlibPtr;

use {util, NAME, components};
use config::{ComponentConfig};
use components::i3workspace::scroll_workspace;

use std::ffi::CString;
use std::os::raw::c_int;

#[derive(Debug)]
pub struct Bar<'a, 'b, 'c> {
    pub config: &'b ComponentConfig,
    pub components: &'c Vec<ComponentConfig>,
    pub application: &'a gtk::Application,
}

impl<'a, 'b, 'c> Bar<'a, 'b, 'c> {
    pub fn new(
        application: &'a gtk::Application,
        config: &'b ComponentConfig,
        components: &'c Vec<ComponentConfig>,
    ) -> Bar<'a, 'b, 'c> {

        let bar = Bar { config, application, components };

        let monitors = util::get_monitors();
        let monitor_index = bar.config.get_int_or("monitor", 0);
        let monitor_option = monitors.get(monitor_index as usize);

        match monitor_option {
            None => {
                eprintln!(
                    "warning: no monitor at index {}",
                    monitor_index,
                );
            },
            Some(monitor) => {
                bar.init(monitor);
            },
        }

        bar
    }


    fn init(&self, monitor: &Rectangle) {

        let window = Window::new(WindowType::Toplevel);
        self.application.add_window(&window);

        // set base values
        window.set_title(NAME);
        window.set_default_size(monitor.width, 1);
        window.set_type_hint(gdk::WindowTypeHint::Dock);
        window.set_wmclass(NAME, NAME);
        window.stick();
        window.set_keep_above(true);

        // attach container
        let container = gtk::Box::new(Orientation::Horizontal, 0);
        WidgetExt::set_name(&container, &self.config.name);
        WidgetExt::set_name(&window, &self.config.name);

        // attach scrollevent
        let monitor_index = self.config.get_int_or("monitor", 0) as i32;
        let viewport = gtk::Viewport::new(None, None);
        // set gdk::EventMask::SCROLL_MASK and disable 'smooth' scrolling
        viewport.add_events(2097152);
        // when scrolling, change workspace
        if self.config.get_bool_or("scroll_workspace", true) {
            viewport.connect_scroll_event(move |_vp, e| {
                let direction = e.get_direction();
                let is_next = direction == ScrollDirection::Down;
                // change workspace (i3)
                scroll_workspace(is_next, monitor_index);
                Inhibit(true)
            });
        }
        viewport.set_shadow_type(gtk::ShadowType::None);
        viewport.add(&container);
        window.add(&viewport);


        // set position
        {
            let position = self.config.get_str_or("position", "top").to_string();
            let &Rectangle { x, y, height, .. } = monitor;
            // TODO: fix 0,0 bug non positioning bug
            window.connect_size_allocate(move |window, rect| {
                let xpos = x;
                let ypos = match position.as_str() {
                    "bottom" => y + (height - rect.height),
                    _ => y,
                };
                // if (xpos, ypos) != window.get_position() {
                    window.move_(xpos, ypos);
                    // println!("{:#?}", rect.height);
                    // wm::util::set_strut(window_role.clone());
                // }
            });
        }

        // show bar
        window.show_all();

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
            let data = [
                0, 0, // left
                0, 0, // right
                32, 0, // top
                0, 0, // bottom
            ];
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
            let data = [
                0, 0, // left
                0, 0, // right
                32, 0, // top
                0, 0, // bottom
                0, 0, // start
                0, 0, // end
                0, 0, // start
                0, 0, // end
                0, 0, // start
                127, 7, // end
                0, 0, // start
                0, 0, // end
            ];
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
    // # 0, 0, bar_size, 0 are the number of pixels to reserve along each edge of the
    // # screen given in the order left, right, top, bottom. Here the size of the bar
    // # is reserved at the top of the screen and the other edges are left alone.
    // #
    // # _NET_WM_STRUT_PARTIAL also supplies a further four pairs, each being a
    // # start and end position for the strut (they don't need to occupy the entire
    // # edge).
        }

        // load components
        components::load_components(&container, &self);

        // TODO: windowEnter set ::focus
        // window.connect_enter_notify_event(move |_, _evt| {
        //     Inhibit(true)
        // });
    }

}
