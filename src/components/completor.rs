use crate::bar::Bar;
use crate::components::Component;
use crate::config::ConfigGroup;
use gtk::prelude::*;
use crate::wm;
use gdk::prelude::*;
use crate::wm::ipc::parser::parse_message;
use crate::wm::events::Event;

pub struct Completor {
    window: gtk::Window,
    wrapper: gtk::Box,
}

impl Component for Completor {
    fn destroy(&self) {
        self.window.destroy();
    }
}

impl Completor {
    pub fn init(config: ConfigGroup, bar: &mut Bar, container: &gtk::Box) {

        let window = gtk::Window::new(gtk::WindowType::Toplevel);
        window.set_type_hint(gdk::WindowTypeHint::PopupMenu);
        window.set_skip_pager_hint(false);
        window.set_skip_taskbar_hint(false);
        window.set_decorated(false);
        window.set_title(&config.name);
        window.set_keep_above(true);
        // let (x, y) = bar.window.get_position();
        // window.move_(x, y);
        // window.show();

        // wm::gtk::set_transparent(&window);
        // wm::gtk::disable_shadow(&window);

        if let Some(ctx) = window.get_style_context() {
            // ctx.add_class("completor");
            // TODO: add class / id from parent
        }
        window.stick();

        let entry = gtk::Entry::new();
        entry.set_has_frame(false);
        entry.show();
        window.add(&entry);
        bar.wm_util.add_window(&window);
        entry.grab_focus();
        entry.set_placeholder_text("hello");

        window.resize(1, 1);
        window.set_resizable(false);
        // println!("{:#?}", window.hide_on_delete());

        let wm_util = bar.wm_util.clone();
        let w_clone = window.clone();
        entry.connect_activate(move |e| {
            if let Some(text) = e.get_text() {
                if let Ok(cmd) = parse_message(&text) {
                    wm_util._run_command(cmd);
                }
            }
        });

        entry.connect_key_press_event(move |e, t| {
            // println!("{:#?}", t);
            gtk::Inhibit(false)
        });

        entry.connect_event(|_, e| {
            // swallow right click
            Inhibit(e.get_button() == Some(3))
        });

        // get focus event

        let event_type = Event::Focus(config.name.clone());
        let event = bar.wm_util.add_listener(event_type, clone!(window
            move |_| {
                window.show();
            }
        ));

        // create wrapper

        let wrapper = gtk::Box::new(gtk::Orientation::Horizontal, 0);

        wrapper.connect_size_allocate(clone!(window move |c, rect| {
            let w = c.get_window().unwrap();
            let (_zo, xo, yo) = w.get_origin();
            window.move_(xo + rect.x, yo + rect.y);
        }));

        super::init_widget(&wrapper, &config, bar, container);

        wrapper.show();
        bar.add_component(Box::new(Completor {
            window,
            wrapper,
        }));
    }
}
