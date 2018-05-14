use super::config::{ComponentConfig};

trait Component {
    fn init(&self, container: gtk::Box, config: ComponentConfig);

    fn new(container: gtk::Box, config: ComponentConfig) {
        // impl
    }
}
