use crate::components::{Component, ComponentParams};
use crate::config::{ConfigGroup, Property};
use gtk;
use gtk::prelude::*;
use gtk::{EventBox, Label, Menu as GtkMenu, MenuItem as GtkMenuItem, WidgetExt};

// gtk context menu

pub struct Dropdown {
    wrapper: EventBox,
}

impl Component for Dropdown {
    fn destroy(&self) {
        self.wrapper.destroy();
    }
}

#[derive(Debug)]
enum MenuItem {
    Command(String, String),
    SubMenu(String, Vec<MenuItem>),
}

// TODO: move to config and refactor config (folders)
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

impl Dropdown {
    pub fn init(params: ComponentParams) {
        let ComponentParams { config, window, container, .. } = params;
        let label = Label::new(None);
        {
            let label_text = config.get_str_or("label", "");
            label.set_markup(&label_text);
        } // config no longer borrowed

        let ebox = EventBox::new();
        ebox.add(&label);
        ebox.show_all();
        super::init_widget(&ebox, &config, &window, container);

        let menu_items = get_menu(config.get_vec_or("items", vec![]));

        let menu = Self::create_menu(&menu_items);

        menu.show_all();

        ebox.connect_button_release_event(clone!(menu move |_c, _e| {
            menu.popup_easy(0, 0);
            Inhibit(false)
        }));

        window.add_component(Box::new(Dropdown {
            wrapper: ebox,
        }));
    }
    fn create_menu(menu_items: &Vec<MenuItem>) -> GtkMenu {
        let menu = GtkMenu::new();
        menu_items.iter().for_each(|item| match item {
            MenuItem::Command(label, command) => {
                let item = GtkMenuItem::new_with_label(label);
                menu.append(&item);
                item.connect_activate(clone!(command move |_| {
                        crate::util::run_command(command.to_string());
                    }));
            }
            MenuItem::SubMenu(label, items) => {
                let submenu = Self::create_menu(items);
                let item = GtkMenuItem::new_with_label(label);
                item.set_submenu(&submenu);
                menu.append(&item);
            }
        });
        menu
    }
}
