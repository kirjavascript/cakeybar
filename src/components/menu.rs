use super::{Component, Bar, gtk, ComponentConfig};
use gtk::prelude::*;
use gtk::{Label, EventBox, Window, WindowType};
use gdk::{EventType, WindowExt, Rectangle};
use std::cell::RefCell;
use std::rc::Rc;

use config::Property;

pub struct Menu {
    // items: Vec<(String, String)>,
    is_open: bool,
    bbox: Rectangle,
}

impl Menu {
    fn toggle(&mut self) {
        self.is_open = !self.is_open;
    }
}


impl Component for Menu {
    fn init(container: &gtk::Box, config: &ComponentConfig, bar: &Bar) {
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
            Menu {
                // items: items,
                bbox: Rectangle { x: 0, y: 0, width: 0, height: 0 },
                is_open: false,
            }
        ));

        let window = Window::new(WindowType::Popup);
        window.set_default_size(100, 200);
        bar.application.add_window(&window);
        window.move_(100,100);
        // get bar position (for under/over)
        //
        // show window

        // track widget bbox
        // let menu_clone = menu.clone();
        ebox.connect_size_allocate(enclose!(menu move |_, rect| {
            menu.borrow_mut().bbox = *rect;
        }));

        ebox.connect_button_press_event(enclose!(window move |c, e| {
            if e.get_event_type() == EventType::ButtonPress {
                menu.borrow_mut().toggle();
                if menu.borrow().is_open {
                    // let w = c.get_window().unwrap();
                    // let (x, y, z) = w.get_origin();

                    window.show();
                }
                else {
                    window.hide();
                }
            }
            Inhibit(false)
        }));

    }
}
