use gtk;
use gtk::{Label, WidgetExt, LabelExt, ContainerExt};

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct LabelGroup {
    widgets: Rc<RefCell<Vec<Label>>>,
    pub wrapper: gtk::Box,
}

impl LabelGroup {
    pub fn new() -> Self {
        let wrapper = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        wrapper.show();
        LabelGroup {
            widgets: Rc::new(RefCell::new(Vec::new())),
            wrapper,
        }
    }

    pub fn set(&self, labels: &Vec<String>) {
        for (i, text) in labels.iter().enumerate() {
            // check if the label exists already
            let added_opt = if let Some(widget) = self.widgets.borrow_mut().get_mut(i) {
                widget.set_text(&text);
                None
            } else {
                // otherwise create a new one
                let widget = Label::new(None);
                self.wrapper.add(&widget);
                widget.set_text(&text);
                widget.show();
                Some(widget)
            };
            // add the new one to the vec
            if let Some(added) = added_opt {
                self.widgets.borrow_mut().push(added);
            }
        }
        // remove unused labels
        let widget_len = self.widgets.borrow().len();
        let label_len = labels.len();
        if widget_len > label_len {
            let mut widgets = self.widgets.borrow_mut();
            widgets.splice(label_len.., vec![]).for_each(|w| {
                w.destroy();
            });
        }
    }

}
