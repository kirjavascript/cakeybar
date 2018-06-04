use super::{Component, Bar, gtk, ComponentConfig};
use gtk::prelude::*;
use gtk::{Label, EventBox};
use std::cell::RefCell;
use std::rc::Rc;

use config::Property;

pub struct Menu<'a, 'b> {
    items: Vec<(&'a String, &'b String)>,
    is_open: bool,
}

impl<'a, 'b> Menu<'a, 'b> {
    fn toggle(&mut self) {
        self.is_open == !self.is_open;
    }
}

impl<'a, 'b> Component for Menu<'a, 'b> {
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
        let default = (&"".to_string(), &"".to_string());
        let items = config.get_vec_or("items", vec![]);
        let items: Vec<(&String, &String)> = items
            .iter()
            .map(|x| {
                if let Property::Array(val) = x {
                    let name_opt = val.get(0);
                    let exec_opt = val.get(1);
                    if let Some(Property::String(name)) = name_opt {
                        if let Some(Property::String(exec)) = exec_opt {
                            return (name, exec);
                        }
                    }
                }
                default
            })
            .filter(|x| x != &default)
            .collect();

        let menu = Rc::new(RefCell::new(
            Menu { items, is_open: false }
        ));

        ebox.connect_button_press_event(move |_, _| {
            // let value = !*menu_open.borrow();
            // *menu_open.borrow_mut() = value;
            // println!("{:#?}", value);
            // menu.borrow_mut().toggle();
            // println!("{:#?}", menu.borrow().is_open);
            Inhibit(false)
        });
        // click event
        //
        // get bar position (for under/over)
        //
        // show window
    }
}
