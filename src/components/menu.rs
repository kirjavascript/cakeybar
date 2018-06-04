use super::{Component, Bar, gtk, ComponentConfig};
use gtk::prelude::*;
use gtk::{Label, EventBox};
use std::cell::RefCell;
use std::rc::Rc;

use config::Property;

pub struct Menu {
    items: Vec<(String, String)>,
    is_open: bool,
}

impl Menu {
    fn toggle(&mut self) {
        self.is_open == !self.is_open;
    }
}

impl Component for Menu {
    fn init(container: &gtk::Box, config: &ComponentConfig, _bar: &Bar) {
        let label = Label::new(None);
        Menu::init_widget(&label, &config);
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
                if let Property::Array(val) = x {
                    let name_opt = val.get(0);
                    let exec_opt = val.get(1);
                    if let Some(Property::String(name)) = name_opt {
                        if let Some(Property::String(exec)) = exec_opt {
                            items.push((name.clone(), exec.clone()));
                        }
                    }
                }
            });

        let menu = Rc::new(RefCell::new(
            Menu { items: items, is_open: false }
        ));

        ebox.connect_button_press_event(move |_, _| {
            // let value = !*menu_open.borrow();
            // *menu_open.borrow_mut() = value;
            // println!("{:#?}", value);
            menu.borrow_mut().toggle();
            println!("{:#?}", menu.borrow().is_open);
            Inhibit(false)
        });
        // click event
        //
        // get bar position (for under/over)
        //
        // show window
    }
}
