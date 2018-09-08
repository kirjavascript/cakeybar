use {gtk, glib};
use gtk::{
    Align,
    WidgetExt,
    StyleContextExt,
    ContainerExt,
    OverlayExt,
};
use bar::Bar;
use config::{ConfigGroup, Property};

mod bandwidth;
mod battery;
mod clock;
mod container;
mod cpu;
mod disk;
mod dropdown;
// mod equalizer;
mod image;
mod ip;
mod memory;
// mod menu;
// mod mode;
// mod script;
// mod tray;
// mod window;
// mod workspaces;

/// interface for all components
pub trait Component {
    /// provide the config object for this component
    fn get_config(&self) -> &ConfigGroup;
    /// show component
    fn show(&mut self);
    /// hide component
    fn hide(&mut self);
    /// clean up any remaining timeouts, callbacks
    fn destroy(&self);
}

/// each component MUST call bar.add_component
pub fn load_component(
    config: ConfigGroup, bar: &mut Bar, container: &gtk::Box
) {
    fn void(config: ConfigGroup, bar: &mut Bar, container: &gtk::Box) {
        warn!("a valid type is required for #{}", config.name)
    }
    // decide which component to load
    (match config.get_str_or("type", "void") {
        "bandwidth" => bandwidth::Bandwidth::init,
        "battery" => battery::Battery::init,
        "clock" => clock::Clock::init,
        "container" => container::Container::init,
        "cpu" => cpu::CPU::init,
        "disk" => disk::Disk::init,
        "dropdown" => dropdown::Dropdown::init,
        // "equalizer" | "equaliser" => equalizer::Equalizer::init,
        "image" => image::Image::init,
        "ip" => ip::IP::init,
        "memory" => memory::Memory::init,
        // "menu" => menu::Menu::init,
        // "mode" => mode::Mode::init,
        // "script" => script::Script::init,
        // "tray" => tray::Tray::init,
        // "window" => window::Window::init,
        // "workspaces" => workspaces::Workspaces::init,
        _ => void,
    }) (config, bar, container);
}

pub fn init_widget<T>(
    widget: &T,
    config: &ConfigGroup,
    bar: &Bar,
    container: &gtk::Box,
) where T: gtk::IsA<gtk::Widget>
        + gtk::IsA<gtk::Object>
        + glib::value::SetValue {
    // TODO: add EventBox
    // TODO: add wrapper
    // let wrapper = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    // wrapper.add(widget);
    // wrapper.show();
    // let widget = &wrapper;
    // set name
    widget.set_name(&config.name);
    // class
    let class_str = config.get_str_or("class", "null");
    if class_str != "null"  {
        if let Some(ctx) = widget.get_style_context() {
            ctx.add_class(class_str);
        }
    }
    let is_fixed = config.get_bool_or("fixed", false);
    // set alignment
    let halign_str = config.get_str_or("halign", "null");
    if halign_str != "null" {
        WidgetExt::set_halign(widget, get_alignment(halign_str));
        if !is_fixed {
            WidgetExt::set_hexpand(widget, true);
        }
    }
    let valign_str = config.get_str_or("valign", "null");
    if valign_str != "null" {
        WidgetExt::set_valign(widget, get_alignment(valign_str));
        if !is_fixed {
            WidgetExt::set_vexpand(widget, true);
        }
    }
    // set layout type
    if is_fixed {
        bar.overlay.add_overlay(widget);
        if config.get_bool_or("pass-through", true) {
            bar.overlay.set_overlay_pass_through(widget, true);
        }
    } else {
        container.add(widget);
    }
}

fn get_alignment(align: &str) -> Align {
    match align {
        "start" => Align::Start,
        "end" => Align::End,
        "center" | "centre" => Align::Center,
        "fill" => Align::Fill,
        _ => Align::Baseline,
    }
}
