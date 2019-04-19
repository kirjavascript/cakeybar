use crate::bar::Bar;
use crate::config::{Config, ConfigGroup, parse_config};
use crate::wm::events::{Event, EventEmitter, EventId, EventValue};
use crate::wm::ipc::commands::*;
use crate::wm::workspace::Workspace;
use crate::wm::watch::Watcher;
use crate::wm::Window as _;
use crate::wm;
use clap::ArgMatches;

use gtk;
use gtk::prelude::*;
use gtk::CssProvider;

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
    windows: Rc<RefCell<Vec<Box<dyn wm::Window>>>>,
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
        let wm_type = if wm::i3::connect().is_ok() {
            WMType::I3
        } else if wm::bsp::connect().is_ok() {
            // TODO: for bspwm, replace detection with checking the root window so we can give
            // better errors
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

        let windows = Rc::new(RefCell::new(Vec::new()));

        let util = WMUtil { data, windows };

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

        wm::gtk::css_reset();
        util.load_theme(None);
        util.load_windows();
        if matches.is_present("watch") {
            util.watch_files();
        }

        util
    }

    pub fn add_gtk_window(&self, window: &gtk::Window) {
        self.data.borrow().app.add_window(window);
    }

    pub fn run_command(&self, cmd: Command) {
        wm::ipc::exec::run_command(self, cmd);
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
                // unload old windows if changing the config
                self.windows.borrow_mut().iter().for_each(|b| b.destroy());
                self.windows.borrow_mut().clear();
                // watch different files
                self.rewatch_files();
            }
            // reload everything
            self.load_theme(None);
            self.load_windows();
        } else if let Err(msg) = config_res {
            error!("{}", msg);
        }
    }

    pub fn load_theme(&self, new_path: Option<String>) {
        // unload old theme
        if let Some(ref provider) = self.data.borrow().css_provider {
            wm::gtk::unload_theme(provider);
            self.windows.borrow().iter().for_each(|window| window.relayout());
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

    fn load_windows(&self) {
        // unload old windows and retain gtk::Window
        let windows = self.windows.borrow_mut().split_off(0);
        let mut gtk_windows: Vec<gtk::Window> =
            windows.iter().map(|b| b.to_window()).collect();
        gtk_windows.reverse();

        // get monitor info
        let monitors = wm::gtk::get_monitor_geometry();
        // clone is here to ensure we're not borrowing during component loading
        let bars = self.data.borrow().config.bars.clone();
        let mut bars: Vec<Box<dyn wm::Window>> =
            bars.iter().fold(Vec::new(), |mut acc, bar_config| {
                let monitor_index = bar_config.get_int_or("monitor", 0);
                let monitor_option = monitors.get(monitor_index as usize);

                if let Some(monitor) = monitor_option {
                    let mut bar = Bar::new(
                        bar_config.clone(),
                        &self,
                        monitor,
                        gtk_windows.pop(),
                    );

                    // load components
                    let container = &bar.get_container().clone();
                    for name in bar_config.get_string_vec("layout") {
                        let config_opt = self.get_component_config(&name);
                        if let Some(config) = config_opt {
                            bar.load_component(
                                config,
                                container,
                                &self
                            );
                        } else {
                            warn!("missing component #{}", name);
                        }
                    }

                    acc.push(Box::new(bar));
                } else {
                    warn!("no monitor at index {}", monitor_index);
                }
                acc
            });

        // destroy old (now unused) windows
        gtk_windows.iter().for_each(|w| w.destroy());
        // update new window vec
        self.windows.borrow_mut().clear();
        self.windows.borrow_mut().append(&mut bars);
    }

    pub fn display_bars(&self, names: &Selectors, show: bool) {
        for bar in self.windows.borrow().iter() {
            if bar.matches_selectors(names) {
                if show {
                    bar.show();
                } else {
                    bar.hide();
                }
            }
        }
    }

    // getters

    pub fn get_wm_type(&self) -> WMType {
        self.data.borrow().wm_type.clone()
    }

    pub fn get_component_config(&self, name: &str) -> Option<ConfigGroup> {
        self.data.borrow().config.components.iter().find(|x| {
            x.name == name
        }).cloned()
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

    pub fn focus_workspace(&self, workspace_name: &str) {
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
        if WMType::Bsp == self.data.borrow().wm_type {
            wm::bsp::set_padding(is_top, padding);
        }
    }
}
