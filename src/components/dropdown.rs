use super::{Component, Bar, gtk, ComponentConfig};
use gtk::prelude::*;
use gtk::{
    Label,
    EventBox,
    WidgetExt,
    Menu,
    MenuItem,
};

// gtk context menu

use config::Property;

pub struct Dropdown { }

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

        let menu = Menu::new();

        items.iter().for_each(|(key, value)| {
            let item = MenuItem::new_with_label(key);
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
