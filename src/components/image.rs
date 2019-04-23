use crate::components::{Component, ComponentParams};
use gtk::prelude::*;
use gtk::Image as GtkImage;

pub struct Image {
    image: GtkImage,
}

impl Component for Image {
    fn destroy(&self) {
        self.image.destroy();
    }
}

impl Image {

    pub fn init(params: ComponentParams) {
        let ComponentParams { config, window, container, wm_util } = params;
        if let Some(src) = config.get_string("src") {
            let img: GtkImage = GtkImage::new_from_file(&wm_util.get_path(&src));
            super::init_widget(&img, &config, &window, container);

            // wait a tick, otherwise we get negative height warnings
            gtk::idle_add(clone!(img move || {
                img.show();
                gtk::Continue(false)
            }));

            window.add_component(Box::new(Image { image: img }));
        } else {
            warn!("#{} missing src property", config.name);
        }
    }
}
