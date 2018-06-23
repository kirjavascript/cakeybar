pub mod atom;
pub mod xcb;
pub mod gtk;
pub mod bsp;

use i3ipc::I3Connection;
use components::i3workspace::scroll_workspace;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub enum WMType {
    I3,
    Unknown,
}

#[derive(Debug)]
pub struct WMUtil {
    data: Rc<RefCell<Data>>,
}

#[derive(Debug)]
struct Data {
    pub wm_type: WMType,
}


impl WMUtil {

    pub fn new() -> Self {
        let data = Rc::new(RefCell::new(Data {
            wm_type: Self::get_wm_type(),
        }));

        WMUtil { data }
    }

    pub fn clone(&self) -> Self {
        WMUtil { data: self.data.clone() }
    }

    fn get_wm_type() -> WMType {
        if let Ok(_) = I3Connection::connect() {
            return WMType::I3
        }

        WMType::Unknown
    }

    pub fn scroll_workspace(&self, forward: bool, monitor_index: i32) {
        match self.data.borrow().wm_type {
            WMType::I3 => {
                scroll_workspace(forward, monitor_index);
            },
            _ => {},
        }
    }

    // pub fn subscribe()
}
