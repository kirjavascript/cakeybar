use super::{gtk, Component, Bar, ComponentConfig, Property};
use gtk::prelude::*;
use gtk::{Image as GtkImage};

pub struct Image { }

impl Component for Image {
    fn init(container: &gtk::Box, config: &ComponentConfig, bar: &Bar) {
        if let Some(&Property::String(ref src)) = config.properties.get("src") {
            let img: GtkImage = GtkImage::new_from_file(
                &bar.app_config.get_path(src)
            );
            Self::init_widget(&img, container, config, bar);

            // wait a tick, otherwise we get negative height warnings
            gtk::idle_add(clone!(img move || {
                img.show();
                gtk::Continue(false)
            }));
        } else {
            warn!("#{} missing src property", config.name);
        }
    }
}
