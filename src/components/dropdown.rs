use super::{Component, Bar, gtk, ConfigGroup};
use gtk::prelude::*;
use gtk::{
    Label,
    EventBox,
    WidgetExt,
    Menu as GtkMenu,
    MenuItem as GtkMenuItem,
};

// gtk context menu

use config::Property;

pub struct Dropdown;

#[derive(Debug)]
enum MenuItem {
    Command(String, String),
    SubMenu(String, Vec<MenuItem>),
}

fn get_menu(items: Vec<Property>) -> Vec<MenuItem> {
    let mut menu_items: Vec<MenuItem> = Vec::new();
    items.iter().for_each(|item| {
        if let Property::Object(obj) = item {
            let name_opt = obj.get("label");
            let exec_opt = obj.get("command");
            let child_opt = obj.get("children");
            if let Some(Property::String(name)) = name_opt {
                if let Some(Property::String(exec)) = exec_opt {
                    let command = MenuItem::Command(name.clone(), exec.clone());
                    menu_items.push(command);
                } else if let Some(Property::Array(children)) = child_opt {
                    let submenu = get_menu(children.to_vec());
                    let children = MenuItem::SubMenu(name.clone(), submenu);
                    menu_items.push(children);
                }
            }
        }
    });
    menu_items
}

impl Component for Dropdown {
    fn init(container: &gtk::Box, config: &ConfigGroup, bar: &Bar) {
        let label = Label::new(None);
        let label_text = config.get_str_or("label", "");
        label.set_markup(&label_text);
        let ebox = EventBox::new();
        ebox.add(&label);
        ebox.show_all();
        Self::init_widget(&ebox, container, config, bar);

        let menu_items = get_menu(config.get_vec_or("items", vec![]));

        let menu = Self::create_menu(&menu_items);

        menu.show_all();

        ebox.connect_button_release_event(clone!(menu move |_c, _e| {
            menu.popup_easy(0, 0);
            Inhibit(false)
        }));
    }
}

impl Dropdown {
    fn create_menu(menu_items: &Vec<MenuItem>) -> GtkMenu {
        let menu = GtkMenu::new();
        menu_items.iter().for_each(|item| {
            match item {
                MenuItem::Command(label, command) => {
                    let item = GtkMenuItem::new_with_label(label);
                    menu.append(&item);
                    item.connect_activate(clone!(command move |_| {
                        ::util::run_command(command.to_string());
                    }));
                },
                MenuItem::SubMenu(label, items) => {
                    let submenu = Self::create_menu(items);
                    let item = GtkMenuItem::new_with_label(label);
                    item.set_submenu(&submenu);
                    menu.append(&item);
                },
            }
        });
        menu
    }
}
