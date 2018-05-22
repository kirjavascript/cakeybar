use super::{gtk, Component, Bar, ComponentConfig, Property};
use gtk::prelude::*;
use gtk::{Image as GtkImage};
use std::path::Path;

pub struct Image { }

impl Component for Image {
    fn init(container: &gtk::Box, config: &ComponentConfig, _bar: &Bar) {
        if let Some(&Property::String(ref src)) = config.properties.get("src") {
            let img: GtkImage = GtkImage::new_from_file(
                Path::new(src)
            );
            Image::init_widget(&img, config);
            container.add(&img);

            // wait a tick, otherwise we get negative height warnings
            let img_clone = img.clone();
            gtk::idle_add(move || {
                img_clone.show();
                gtk::Continue(false)
            });
        }
    }
}
