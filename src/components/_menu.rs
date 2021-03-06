use super::{Component, Bar, gtk, ConfigGroup};
use gtk::prelude::*;
use gtk::{
    Label,
    EventBox,
    Window,
    WindowType,
    Orientation,
    WidgetExt,
};
use gdk::{
    EventType,
    WindowExt,
// WindowTypeHint,
    Rectangle,
};

// 'start' style menu

use std::cell::RefCell;
use std::rc::Rc;

use config::Property;

pub struct Menu {
    is_open: bool,
    bbox: Rectangle,
}

impl Component for Menu {
    fn init(container: &gtk::Box, config: &ConfigGroup, bar: &Bar) {
        let label = Label::new(None);
        let label_text = config.get_str_or("label", "");
        label.set_markup(&label_text);
        let ebox = EventBox::new();
        ebox.add(&label);
        Self::init_widget(&ebox, container, config, bar);
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

        let menu = Rc::new(RefCell::new(
            Menu {
                bbox: Rectangle { x: 0, y: 0, width: 0, height: 0 },
                is_open: false,
            }
        ));

        let window = Window::new(WindowType::Popup);
        window.set_default_size(10, 10);
        // window.set_decorated(false);
        // window.set_skip_pager_hint(false);
        // window.set_skip_taskbar_hint(false);
        // window.set_type_hint(WindowTypeHint::Utility);

        // window.connect_focus_out_event(clone!((window, menu) move |_, _| {
        //     menu.borrow_mut().is_open = false;
        //     window.hide();
        //     Inhibit(false)
        // }));
        bar.application.add_gtk_window(&window);
        // TODO: get bar position (for under/over)
        // TODO: get alignment (set for text and popup position)

        // add items to exec
        let wrapper = gtk::Box::new(Orientation::Vertical, 0);
        for (name, exec) in items {
            let ebox = EventBox::new();
            let label = Label::new(None);
            label.set_markup(&name);
            ebox.add(&label);
            wrapper.add(&ebox);

            // run command on click
            ebox.connect_button_press_event(clone!((window, menu) move |_, _| {
                ::util::run_command(exec.to_string());
                // toggle menu
                menu.borrow_mut().is_open = false;
                window.hide();
                Inhibit(false)
            }));
        }
        wrapper.show_all();
        window.add(&wrapper);

        // set window id
        let window_id = config.get_str_or("window_id", "void");
        if window_id != "void" {
            WidgetExt::set_name(&wrapper, &window_id);
            WidgetExt::set_name(&window, &window_id);
        }

        // track widget bbox
        ebox.connect_size_allocate(clone!(menu move |_, rect| {
            menu.borrow_mut().bbox = *rect;
        }));

        ebox.connect_button_press_event(clone!(window move |c, e| {
            if e.get_event_type() == EventType::ButtonPress {
                if !menu.borrow().is_open {
                    let bbox = menu.borrow().bbox;
                    let w = c.get_window().unwrap();
                    let (_z, x, y) = w.get_origin();
                    window.show();
                    window.move_(x, y + bbox.height);
                    // window.grab_focus();
                    menu.borrow_mut().is_open = true;
                }
                else {
                    window.hide();
                    menu.borrow_mut().is_open = false;
                }
            }
            Inhibit(false)
        }));

    }
}
