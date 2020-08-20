use crate::Key;

/// A variable that can be resolved by an injector.
pub struct Dependency {
    key: Key,
    // TODO: optional
    parameter_index: i8,
}

impl Dependency {
    pub fn new(key: Key, parameter_index: i8) -> Self {
        Self {
            key,
            parameter_index,
        }
    }

    pub fn key(&self) -> &Key {
        &self.key
    }

    pub fn parameter_index(&self) -> i8 {
        self.parameter_index
    }
}
