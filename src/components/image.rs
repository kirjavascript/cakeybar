use super::{gtk, Component, Bar, ComponentConfig};
use gtk::prelude::*;
use gtk::{Image as GtkImage};
use std::path::Path;

pub struct Image {
}

impl Component for Image {
    fn init(container: &gtk::Box, config: &ComponentConfig, bar: &Bar) {
        let img: GtkImage = GtkImage::new_from_file(
            Path::new("./example/icon.svg")
        );
        WidgetExt::set_name(&img, "icon");
        container.add(&img);
    }
}
