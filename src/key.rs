//! Defines the key handlers

use std::collections::HashMap;

use crate::AppInterface;
use crate::interface::ReturnStatus;
use crate::runner::Action;

/// All the keys that can be overridden
#[non_exhaustive]
#[derive(PartialEq, Eq, Hash)]
pub enum Key {
    /// Enter key
    Escape,
}

pub type KeyPress = Box<dyn Action>;

/// Contains all the defined handlers for a given set of keys
pub struct Keys(HashMap<Key, KeyPress>);

impl Keys {
    /// Define an action for a key
    pub fn define_key(&mut self, key: Key, action: impl Action + 'static) -> Option<()> {
        self.0.insert(key, Box::new(action)).map(|_old| ())
    }

    /// Fire an action after [`Key`] was pressed
    pub fn fire_key(&mut self, key: &Key, line: &str) -> ReturnStatus {
        self.0.get_mut(key).map_or(ReturnStatus::None, |handler| {
            let mut app = AppInterface::new(line);
            handler(&mut app);
            app.take_status()
        })
    }

    /// Creates a default [`Keys`]
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}
