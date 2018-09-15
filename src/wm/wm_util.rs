use bar::Bar;
use config::{Config, ConfigGroup, parse_config};
use wm::events::{Event, EventEmitter, EventId, EventValue};
use wm::ipc::commands::*;
use wm::workspace::Workspace;
use wm::watch::Watcher;
use clap::ArgMatches;

use gtk;
use gtk::prelude::*;
use gtk::CssProvider;
use wm;

use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

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

#[derive(Clone)]
pub struct WMUtil{
    data: Rc<RefCell<Data>>,
    bars: Rc<RefCell<Vec<Bar>>>,
}

struct Data {
    app: gtk::Application,
    config: Config,
    css_provider: Option<CssProvider>,
    events: EventEmitter<Event, EventValue>,
    watcher: Option<Watcher>,
    wm_type: WMType,
}

impl WMUtil {
    pub fn new(
        app: gtk::Application, config: Config, matches: &ArgMatches
    ) -> Self {
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
            config,
            css_provider: None,
            events,
            watcher: None,
            wm_type,
        }));

        let bars = Rc::new(RefCell::new(Vec::new()));

        let util = WMUtil { data, bars };

        // start IPC
        if util.data.borrow().config.global.get_bool_or("enable-ipc", true) {
            wm::ipc::listen(&util);
        }

        // listen for WM events
        wm::xcb::listen(&util);

        // WM specific listeners
        match util.get_wm_type() {
            WMType::I3 => {
                wm::i3::listen(&util);
            }
            WMType::Bsp => {
                wm::bsp::listen(&util);
            }
            _ => {}
        }

        util.load_theme(None);
        util.load_bars();
        if matches.is_present("watch") {
            util.watch_files();
        }

        util
    }

    pub fn add_window(&self, window: &gtk::Window) {
        self.data.borrow().app.add_window(window);
    }

    pub fn watch_files(&self) {
        let watcher = Watcher::new(self, &self.data.borrow().config);
        self.data.borrow_mut().watcher = Some(watcher);
    }

    pub fn rewatch_files(&self) {
        let was_some = {
            let watcher_opt = &self.data.borrow().watcher;
            if let Some(watcher) = watcher_opt {
                watcher.unwatch();
            }
            watcher_opt.is_some()
        };
        if was_some {
            self.watch_files();
        }
    }

    pub fn reload_config(&self, new_path: Option<String>) {
        let change_config = new_path.is_some();
        // update filename
        if let Some(new_path) = new_path {
            self.data.borrow_mut().config.set_filename(new_path);
        }
        // get filename
        let filename = self.data.borrow().config.get_filename();
        // load config
        let config_res = parse_config(&filename);
        if let Ok(config) = config_res {
            // update config
            self.data.borrow_mut().config = config;
            if change_config {
                // unload old bars if changing the config
                self.bars.borrow_mut().iter().for_each(|b| b.destroy());
                self.bars.borrow_mut().clear();
                // watch different files
                self.rewatch_files();
            }
            // reload everything
            self.load_theme(None);
            self.load_bars();
        } else if let Err(msg) = config_res {
            error!("{}", msg);
        }
    }

    pub fn load_theme(&self, new_path: Option<String>) {
        // unload old theme
        if let Some(ref provider) = self.data.borrow().css_provider {
            wm::gtk::unload_theme(provider);
            self.bars.borrow().iter().for_each(|bar| bar.relayout());
        }
        // update path
        if let Some(new_path) = new_path {
            self.data.borrow_mut().config.set_theme(new_path);
        }
        // get theme
        let theme = self.data.borrow().config.get_theme();
        // load new theme
        match wm::gtk::load_theme(&theme) {
            Ok(provider) => {
                self.data.borrow_mut().css_provider = Some(provider);
            }
            Err(err) => {
                error!("{}", err);
            }
        }
    }

    pub fn load_bars(&self) {
        // unload old bars and retain gtk::Window
        let bars = self.bars.borrow_mut().split_off(0);
        let mut windows: Vec<gtk::Window> =
            bars.iter().map(|b| b.to_window()).collect();
        windows.reverse();

        // get monitor info
        let monitors = wm::gtk::get_monitor_geometry();
        // clone is here to ensure we're not borrowing during Bar::load_components
        let bars = self.data.borrow().config.bars.clone();
        let mut bars = bars.iter().fold(Vec::new(), |mut acc, bar_config| {
            let monitor_index = bar_config.get_int_or("monitor", 0);
            let monitor_option = monitors.get(monitor_index as usize);

            if let Some(monitor) = monitor_option {
                acc.push(Bar::new(
                    bar_config.clone(),
                    self.clone(),
                    monitor,
                    windows.pop(),
                ));
            } else {
                warn!("no monitor at index {}", monitor_index);
            }
            acc
        });

        // destroy old (now unused) windows
        windows.iter().for_each(|w| w.destroy());
        // update new bar vec
        self.bars.borrow_mut().clear();
        self.bars.borrow_mut().append(&mut bars);
    }

    pub fn display_bars(&self, names: Vec<String>, show: bool) {
        for bar in self.bars.borrow().iter() {
            if names.contains(&bar.config.name) {
                if show {
                    bar.show();
                } else {
                    bar.hide();
                }
            }
        }
    }

    pub fn display_components(
        &self, bar_names: Vec<String>, selectors: Selectors, show: bool
    ) {
        for bar in self.bars.borrow().iter() {
            if bar_names.len() == 0 || bar_names.contains(&bar.config.name) {
                bar.display_components(&selectors, show);
            }
        }
    }

    // getters

    pub fn get_bar_names(&self) -> Vec<String> {
        self.bars.borrow().iter().map(|x| x.config.name.clone()).collect()
    }

    pub fn get_wm_type(&self) -> WMType {
        self.data.borrow().wm_type.clone()
    }

    pub fn get_component_config(&self, name: &str) -> Option<ConfigGroup> {
        self.data.borrow().config.components.iter().find(|x| {
            &x.name == name
        }) .map(|x| x.clone())
    }

    pub fn get_path(&self, filename: &str) -> String {
        self.data.borrow().config.get_path(filename)
    }

    // events

    pub fn add_listener<F: 'static>(&self, event: Event, callback: F) -> EventId
    where
        F: Fn(Option<EventValue>),
    {
        self.data.borrow_mut().events.add_listener(event, callback)
    }

    pub fn remove_listener(&self, event: Event, id: EventId) {
        self.data.borrow_mut().events.remove_listener(event, id);
    }

    #[allow(dead_code)]
    pub fn emit(&self, event: Event) {
        self.data.borrow().events.emit(event);
    }

    pub fn emit_value(&self, event: Event, value: EventValue) {
        self.data.borrow().events.emit_value(event, value);
    }

    // wm actions

    pub fn get_workspaces(&self) -> Option<Vec<Workspace>> {
        match self.data.borrow().wm_type {
            WMType::I3 => match wm::i3::connect() {
                Ok(mut connection) => Some(wm::i3::get_workspaces(&mut connection)),
                Err(_) => None,
            },
            WMType::Bsp => match wm::bsp::connect() {
                Ok(mut connection) => Some(wm::bsp::get_workspaces(&mut connection)),
                Err(_) => None,
            },
            _ => None,
        }
    }

    pub fn focus_workspace(&self, workspace_name: &String) {
        match self.data.borrow().wm_type {
            WMType::I3 => {
                let command = format!("workspace {}", workspace_name);
                wm::i3::run_command(&command);
            }
            WMType::Bsp => {
                let command = format!("desktop -f {}", workspace_name);
                wm::bsp::run_command(command).ok();
            }
            _ => {}
        }
    }

    pub fn cycle_workspace(&self, forward: bool, monitor_index: i32) {
        match self.data.borrow().wm_type {
            WMType::I3 => {
                wm::i3::cycle_workspace(forward, monitor_index);
            }
            WMType::Bsp => {
                wm::bsp::cycle_workspace(forward, monitor_index);
            }
            _ => {}
        }
    }

    pub fn set_padding(&self, is_top: bool, padding: i32) {
        match self.data.borrow().wm_type {
            WMType::Bsp => {
                wm::bsp::set_padding(is_top, padding);
            }
            // don't need to do this in i3
            _ => {}
        }
    }
}
