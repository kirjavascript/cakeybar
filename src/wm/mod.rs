pub mod atom;
pub mod xcb;
pub mod gtk;
pub mod bsp;
pub mod i3;
pub mod events;
pub mod workspace;

use self::events::{Event, EventValue, EventEmitter};
use self::workspace::Workspace;

use std::cell::RefCell;
use std::rc::Rc;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum WMType {
    I3,
    Bsp,
    Unknown,
}

impl fmt::Display for WMType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("{:?}", &self).to_lowercase())
    }
}

pub struct WMUtil {
    data: Rc<RefCell<Data>>,
}

struct Data {
    wm_type: WMType,
    events: EventEmitter<Event, EventValue>,
}

impl WMUtil {

    pub fn new() -> Self {
        let wm_type = if let Ok(_) = i3::connect() {
            WMType::I3
        } else if let Ok(_) = bsp::connect() {
            WMType::Bsp
        } else {
            WMType::Unknown
        };

        if wm_type != WMType::Unknown {
            info!("detected {}wm", wm_type);
        }

        let events = EventEmitter::new();

        let data = Rc::new(RefCell::new(Data {
            wm_type,
            events,
        }));

        let util = Self { data };

        match util.get_wm_type() {
            WMType::I3 => {
                i3::listen(&util);
            },
            _ => {},
        }

        util
    }

    pub fn clone(&self) -> Self {
        Self { data: self.data.clone() }
    }

    // getters

    pub fn get_wm_type(&self) -> WMType {
        self.data.borrow().wm_type.clone()
    }

    // events

    pub fn add_listener<F: 'static>(&self, event: Event, callback: F)
        where F: Fn(Option<EventValue>) {
        self.data.borrow_mut().events.add_listener(event, callback);
    }

    pub fn emit(&self, event: Event) {
        self.data.borrow().events.emit(event);
    }

    pub fn emit_value(&self, event: Event, value: EventValue) {
        self.data.borrow().events.emit_value(event, value);
    }

    // wm actions

    pub fn get_workspaces(&self) -> Option<Vec<Workspace>> {
        match self.data.borrow().wm_type {
            WMType::I3 => {
                match i3::connect() {
                    Ok(mut connection) => {
                        Some(i3::get_workspaces(&mut connection))
                    },
                    Err(_) => None
                }
            },
            _ => None
        }
    }

    // pub fn focus_workspace

    pub fn cycle_workspace(&self, forward: bool, monitor_index: i32) {
        match self.data.borrow().wm_type {
            WMType::I3 => {
                i3::cycle_workspace(forward, monitor_index);
            },
            _ => {},
        }
    }

    pub fn set_padding(&self, is_top: bool, padding: i32) {
        match self.data.borrow().wm_type {
            WMType::Bsp => {
                bsp::set_padding(is_top, padding);
            },
            // don't need to do this in i3
            _ => {},
        }
    }

}
