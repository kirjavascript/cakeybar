use gtk;
use config::{ConfigGroup, Property};
use components::Component;
use bar::Bar;

pub struct Void;

impl Component for Void {
    fn show(&self){}
    fn hide(&self){}
    fn destroy(&self){}
}

impl Void {
    pub fn init(config: ConfigGroup) -> Box<Self> {
    let type_opt = config.properties.get("type");
    if let Some(&Property::String(ref type_)) = type_opt {
        warn!("{} is not a valid component type", type_);
    } else {
        warn!("a type property is required for components");
    }
        Box::new(Void { })
    }
}
