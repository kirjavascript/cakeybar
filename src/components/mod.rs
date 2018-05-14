use super::config::{ComponentConfig};

mod clock;
mod image;

trait Component {
    fn init(&self, container: gtk::Box, config: ComponentConfig);

    fn new(container: gtk::Box, config: ComponentConfig) {
        // impl
    }
}
