use wm::workspace::Workspace;

// data

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum Event {
    Window,
    Mode,
    Workspace,
}

#[derive(Debug, Clone)]
pub enum EventValue {
    String(String),
    Workspaces(Vec<Workspace>),
}

// impl

use std::collections::HashMap;
use std::hash::Hash;

pub struct EventEmitter<T: Hash + Eq, V: Clone> {
    listeners: HashMap<T, Vec<Box<Fn(Option<V>)>>>,
}

impl<T, V> EventEmitter<T, V> where T: Hash + Eq, V: Clone {
    pub fn new() -> Self {
        Self {
            listeners: HashMap::new(),
        }
    }

    pub fn add_listener<F: 'static>(&mut self, event: T, callback: F)
        where F: Fn(Option<V>) {
        if self.listeners.contains_key(&event) {
            self.listeners.get_mut(&event).unwrap().push(Box::new(callback));
        } else {
            self.listeners.insert(event, vec![Box::new(callback)]);
        }
    }

    #[allow(dead_code)]
    pub fn emit(&self, event: T) {
        if let Some(callbacks) = self.listeners.get(&event) {
            for callback in callbacks {
                callback(None);
            }
        }
    }

    pub fn emit_value(&self, event: T, value: V) {
        if let Some(callbacks) = self.listeners.get(&event) {
            for callback in callbacks {
                callback(Some(value.clone()));
            }
        }
    }
}
