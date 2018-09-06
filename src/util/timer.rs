use gtk;
use glib_sys::g_source_remove;
use glib::translate::ToGlib;
use glib::source::SourceId;

pub struct Timer(u32);

impl Timer {
    pub fn add_seconds<F>(interval: u32, mut callback: F) -> Self
        where F: FnMut() -> gtk::Continue + 'static
    {
        callback();
        Self::from(gtk::timeout_add_seconds(interval, callback))
    }

    pub fn add_ms<F>(interval: u32, mut callback: F) -> Self
        where F: FnMut() -> gtk::Continue + 'static
    {
        callback();
        Self::from(gtk::timeout_add(interval, callback))
    }

    pub fn from(src: SourceId) -> Self {
        Timer(src.to_glib())
    }

    pub fn remove(&self) {
        unsafe {
            g_source_remove(self.0);
        }
    }
}
