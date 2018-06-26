use std::collections::HashMap;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum Event {
    Window,
    Mode,
    Workspace,
}

pub struct EventEmitter {
    listeners: HashMap<Event, Vec<Box<Fn()>>>,
}

impl EventEmitter {
    pub fn new() -> Self {
        Self {
            listeners: HashMap::new(),
        }
    }

    pub fn add_listener<F: 'static>(&mut self, event: Event, callback: F)
        where F: Fn() {
        if self.listeners.contains_key(&event) {
            self.listeners.get_mut(&event).unwrap().push(Box::new(callback));
        } else {
            self.listeners.insert(event, vec![Box::new(callback)]);
        }
    }

    pub fn emit(&self, event: Event) {
        if let Some(callbacks) = self.listeners.get(&event) {
            for callback in callbacks {
                callback();
            }
        }
    }
}
