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

pub type EventId = u32;

pub struct EventEmitter<T: Hash + Eq, V: Clone> {
    listeners: HashMap<T, Vec<(EventId, Box<Fn(Option<V>)>)>>,
    next_id: EventId,
}

impl<T, V> EventEmitter<T, V>
where
    T: Hash + Eq,
    V: Clone,
{
    pub fn new() -> Self {
        Self {
            listeners: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn add_listener<F: 'static>(&mut self, event: T, callback: F) -> EventId
    where
        F: Fn(Option<V>),
    {
        let id = self.next_id;
        self.next_id += 1;
        let listener = (id, Box::new(callback) as Box<_>);
        if self.listeners.contains_key(&event) {
            self.listeners.get_mut(&event).unwrap().push(listener);
        } else {
            self.listeners.insert(event, vec![listener]);
        }

        id
    }

    pub fn remove_listener(&mut self, event: T, id: EventId) {
        if let Some(listeners) = self.listeners.get_mut(&event) {
            let index_opt = listeners.iter().position(|listener| listener.0 == id);
            if let Some(index) = index_opt {
                listeners.remove(index);
            } else {
                error!("removing non existant event id");
            }
        } else {
            error!("removing non existant event type");
        }
    }

    #[allow(dead_code)]
    pub fn emit(&self, event: T) {
        if let Some(callbacks) = self.listeners.get(&event) {
            for (_, callback) in callbacks {
                callback(None);
            }
        }
    }

    pub fn emit_value(&self, event: T, value: V) {
        if let Some(callbacks) = self.listeners.get(&event) {
            for (_, callback) in callbacks {
                callback(Some(value.clone()));
            }
        }
    }
}
