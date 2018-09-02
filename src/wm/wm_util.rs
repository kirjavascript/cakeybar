use bar::Bar;
use config::{Config, ConfigGroup};
use wm::events::{Event, EventValue, EventEmitter};
use wm::workspace::Workspace;

use gtk;
use wm;
use gtk::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;
use std::fmt;
use std::mem;

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

pub struct WMUtil(Rc<RefCell<Data>>);

struct Data {
    app: gtk::Application,
    bars: Vec<Bar>,
    config: Config,
    events: EventEmitter<Event, EventValue>,
    wm_type: WMType,
}

impl Clone for WMUtil {
    fn clone(&self) -> Self {
        WMUtil(self.0.clone())
    }
}

impl WMUtil {
    pub fn new(app: gtk::Application, config: Config) -> Self {
        let wm_type = if let Ok(_) = wm::i3::connect() {
            WMType::I3
        } else if let Ok(_) = wm::bsp::connect() {
            WMType::Bsp
        } else {
            WMType::Unknown
        };

        if wm_type != WMType::Unknown {
            info!("detected {}wm", wm_type);
        }

        let events = EventEmitter::new();

        let data = Rc::new(RefCell::new(Data {
            app,
            wm_type,
            events,
            config,
            bars: Vec::new(),
        }));

        let util = WMUtil(data);

        // start IPC
        if util.0.borrow().config.global.get_bool_or("enable-ipc", true) {
            wm::ipc::listen(&util);
        }

        // listen for WM events
        wm::xcb::listen(&util);

        // WM specific listeners
        match util.get_wm_type() {
            WMType::I3 => {
                wm::i3::listen(&util);
            },
            WMType::Bsp => {
                wm::bsp::listen(&util);
            },
            _ => {},
        }

        util.load_theme();
        util.load_bars();

        util
    }

    pub fn add_window(&self, window: &gtk::Window) {
        self.0.borrow().app.add_window(window);
    }

    pub fn load_theme(&self) {
        match self.0.borrow().config.get_theme() {
            Some(ref src) => wm::gtk::load_theme(src),
            None => {/* default theme */},
        }
    }

    pub fn load_bars(&self) {
        let monitors = wm::gtk::get_monitor_geometry();
        let bars = self.0.borrow().config.bars.iter()
            .fold(Vec::new(), |mut acc, bar_config| {
                let monitor_index = bar_config.get_int_or("monitor", 0);
                let monitor_option = monitors.get(monitor_index as usize);

                if let Some(monitor) = monitor_option {
                    acc.push(Bar::new(
                        bar_config.clone(),
                        self.clone(),
                        monitor,
                    ));
                } else {
                    warn!("no monitor at index {}", monitor_index);
                }
                acc
            });
        let _ = mem::replace(&mut self.0.borrow_mut().bars, bars);
    }

    pub fn unload_bars(&self) {
        self.0.borrow_mut().bars.iter().for_each(|bar| bar.destroy());
        self.0.borrow_mut().bars.clear();
    }

    // getters

    pub fn get_wm_type(&self) -> WMType {
        self.0.borrow().wm_type.clone()
    }

    pub fn get_component_config(&self, name: &str) -> Option<ConfigGroup> {
        self.0.borrow().config.components.iter().find(|x| {
            &x.name == name
        }).map(|x| x.clone())
    }

    // events

    pub fn add_listener<F: 'static>(&self, event: Event, callback: F)
        where F: Fn(Option<EventValue>) {
        self.0.borrow_mut().events.add_listener(event, callback);
    }

    #[allow(dead_code)]
    pub fn emit(&self, event: Event) {
        self.0.borrow().events.emit(event);
    }

    pub fn emit_value(&self, event: Event, value: EventValue) {
        self.0.borrow().events.emit_value(event, value);
    }

    // wm actions

    pub fn get_workspaces(&self) -> Option<Vec<Workspace>> {
        match self.0.borrow().wm_type {
            WMType::I3 => {
                match wm::i3::connect() {
                    Ok(mut connection) => {
                        Some(wm::i3::get_workspaces(&mut connection))
                    },
                    Err(_) => None
                }
            },
            WMType::Bsp => {
                match wm::bsp::connect() {
                    Ok(mut connection) => {
                        Some(wm::bsp::get_workspaces(&mut connection))
                    },
                    Err(_) => None
                }
            },
            _ => None
        }
    }

    pub fn focus_workspace(&self, workspace_name: &String) {
        match self.0.borrow().wm_type {
            WMType::I3 => {
                let command = format!("workspace {}", workspace_name);
                wm::i3::run_command(&command);
            },
            WMType::Bsp => {
                let command = format!("desktop -f {}", workspace_name);
                wm::bsp::run_command(command).ok();
            },
            _ => {},
        }
    }

    pub fn cycle_workspace(&self, forward: bool, monitor_index: i32) {
        match self.0.borrow().wm_type {
            WMType::I3 => {
                wm::i3::cycle_workspace(forward, monitor_index);
            },
            WMType::Bsp => {
                wm::bsp::cycle_workspace(forward, monitor_index);
            },
            _ => {},
        }
    }

    pub fn set_padding(&self, is_top: bool, padding: i32) {
        match self.0.borrow().wm_type {
            WMType::Bsp => {
                wm::bsp::set_padding(is_top, padding);
            },
            // don't need to do this in i3
            _ => {},
        }
    }

}
