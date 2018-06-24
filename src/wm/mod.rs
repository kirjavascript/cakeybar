pub mod atom;
pub mod xcb;
pub mod gtk;
pub mod bsp;

use i3ipc::I3Connection;
use components::i3workspace;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Clone)]
pub enum WMType {
    I3,
    Bsp,
    Unknown,
}

#[derive(Debug)]
pub struct WMUtil {
    data: Rc<RefCell<Data>>,
}

#[derive(Debug)]
struct Data {
    wm_type: WMType,
}


impl WMUtil {

    pub fn new() -> Self {
        let i3conn = I3Connection::connect();
        let wm_type = if let Ok(_) = i3conn {
            WMType::I3
        } else if let Ok(_) = bsp::connect() {
            WMType::Bsp
        } else {
            WMType::Unknown
        };

        if wm_type != WMType::Unknown {
            let wm_name = format!("{:?}", wm_type);
            info!("detected {}wm", wm_name.to_lowercase());
        }

        let data = Rc::new(RefCell::new(Data {
            wm_type,
        }));

        WMUtil { data }
    }

    pub fn clone(&self) -> Self {
        WMUtil { data: self.data.clone() }
    }

    pub fn get_wm_type(&self) -> WMType {
        self.data.borrow().wm_type.clone()
    }

    pub fn scroll_workspace(&self, forward: bool, monitor_index: i32) {
        match self.data.borrow().wm_type {
            WMType::I3 => {
                i3workspace::scroll_workspace(forward, monitor_index);
            },
            _ => {},
        }
    }

    pub fn set_padding(&self, is_top: bool, padding: i32) {
        match self.data.borrow().wm_type {
            WMType::Bsp => {
                bsp::set_padding(is_top, padding);
            },
            _ => {},
        }
    }

    // pub fn subscribe()
}
