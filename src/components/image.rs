use bar::Bar;
use components::Component;
use config::ConfigGroup;
use gtk;
use gtk::prelude::*;
use gtk::Image as GtkImage;

pub struct Image;

pub struct Bandwidth {
    config: ConfigGroup,
    image: GtkImage,
}

impl Component for Bandwidth {
    fn get_config(&self) -> &ConfigGroup {
        &self.config
    }
    fn show(&mut self) {
        self.image.show();
    }
    fn hide(&mut self) {
        self.image.hide();
    }
    fn destroy(&self) {
        self.image.destroy();
    }
}

impl Image {
    pub fn init(config: ConfigGroup, bar: &mut Bar, container: &gtk::Box) {
        if let Some(src) = config.get_string("src") {
            let img: GtkImage = GtkImage::new_from_file(&bar.wm_util.get_path(&src));
            super::init_widget(&img, &config, bar, container);

            // wait a tick, otherwise we get negative height warnings
            gtk::idle_add(clone!(img move || {
                img.show();
                gtk::Continue(false)
            }));

            bar.add_component(Box::new(Bandwidth { config, image: img }));
        } else {
            warn!("#{} missing src property", config.name);
        }
    }
}
