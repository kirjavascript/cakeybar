use bar::Bar;
use components::Component;
use config::ConfigGroup;
use gtk;
use gdk;
use gtk::prelude::*;

pub struct Completor {
    config: ConfigGroup,
}

impl Component for Completor {
    fn get_config(&self) -> &ConfigGroup {
        &self.config
    }
    fn show(&self) {
        // self.image.show();
    }
    fn hide(&self) {
        // self.image.hide();
    }
    fn destroy(&self) {
        // self.image.destroy();
    }
}

impl Completor {
    pub fn init(config: ConfigGroup, bar: &mut Bar, container: &gtk::Box) {

        // TODO: transparency

        let window = gtk::Window::new(gtk::WindowType::Toplevel);
        window.set_type_hint(gdk::WindowTypeHint::Utility);
        window.set_skip_pager_hint(false);
        window.set_skip_taskbar_hint(false);
        window.set_default_size(1, 1);
        window.set_decorated(false);
        window.set_title(&config.name);
        window.stick();
        // super::init_widget(&wrapper, &config, bar, container);

        let entry = gtk::Entry::new();
        entry.set_has_frame(false);
        entry.show();
        window.show();
        window.add(&entry);
        bar.wm_util.add_window(&window);
        entry.grab_focus();
        entry.connect_activate(move |e| {
            println!("{:#?}", e.get_text());
        });

        entry.connect_key_press_event(move |e, t| {
            println!("{:#?}", t);
            gtk::Inhibit(false)
        });

        window.connect_focus_out_event(move |w, _| {
            // println!("{:#?}", "out");
            // w.grab_focus();
            // w.show();
            // entry.grab_focus();
            w.destroy();
            gtk::Inhibit(false)
        });

        // get window events


        // let d = gtk::Dialog::new();
        // d.show();
        // window.add(&d);


        // let entry = gtk::Entry::new();
        // super::init_widget(&entry, &config, bar, container);
        // entry.show();
        // entry.grab_focus();

        bar.add_component(Box::new(Completor {
            config,
        }));
    }
}
