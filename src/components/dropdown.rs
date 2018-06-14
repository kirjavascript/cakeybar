use super::{Component, Bar, gtk, ComponentConfig};
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

pub struct Dropdown { }

// #[derive(Debug)]
// enum MenuItem {
//     Command(String, String),
//     Children(String, Vec<MenuItem>),
// }

// fn get_menu(items: Vec<Property>) -> Vec<MenuItem> {
//     let mut menu_items: Vec<MenuItem> = Vec::new();
//     items.iter().for_each(|item| {
//         if let Property::Object(obj) = item {
//             let name_opt = obj.get("label");
//             let exec_opt = obj.get("command");
//             let child_opt = obj.get("children");
//             if let Some(Property::String(name)) = name_opt {
//                 if let Some(Property::String(exec)) = exec_opt {
//                     let command = MenuItem::Command(name.clone(), exec.clone());
//                     menu_items.push(command);
//                 } else if let Some(Property::Array(children)) = child_opt {
//                     let submenu = get_menu(children.to_vec());
//                     let children = MenuItem::Children(name.clone(), submenu);
//                     menu_items.push(children);
//                 }
//             }
//         }
//     });
//     menu_items
// }

impl Component for Dropdown {
    fn init(container: &gtk::Box, config: &ComponentConfig, _bar: &Bar) {
        let label = Label::new(None);
        Dropdown::init_widget(&label, &config);
        let label_text = config.get_str_or("label", "");
        label.set_text(&label_text);
        let ebox = EventBox::new();
        ebox.add(&label);
        container.add(&ebox);
        ebox.show_all();

        // let items = get_menu(config.get_vec_or("items", vec![]));
        // println!("{:#?}", items);

        // get list of items
        let mut items: Vec<(String, String)> = Vec::new();
        config.get_vec_or("items", vec![])
            .iter()
            .for_each(|x| {
                if let Property::Object(obj) = x {
                    let name_opt = obj.get("label");
                    let exec_opt = obj.get("command");
                    if let Some(Property::String(name)) = name_opt {
                        if let Some(Property::String(exec)) = exec_opt {
                            items.push((name.clone(), exec.clone()));
                        }
                    }
                }
            });

        let menu = GtkMenu::new();

        items.iter().for_each(|(key, value)| {
            let item = GtkMenuItem::new_with_label(key);
            menu.append(&item);
            item.connect_activate(enclose!(value move |_| {
                ::util::run_command(value.to_string());
            }));
        });

        menu.show_all();

        ebox.connect_button_release_event(enclose!(menu move |_c, _e| {
            menu.popup_easy(0, 0);
            Inhibit(false)
        }));

    }
}
