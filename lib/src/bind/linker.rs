use crate::meta::Binding;
use std::collections::HashMap;
use crate::Key;
use crate::errors::{Errors, ChassisError};

pub struct Linker {
    bindings: HashMap<Key, Binding>,
    errors: Errors,
}

pub struct LinkedBindings {
    bindings: HashMap<Key, Binding>,
}

impl LinkedBindings {
    pub fn bindings(self) -> HashMap<Key, Binding> {
        self.bindings
    }
}

impl Linker {
    pub fn new(bindings: Vec<Binding>) -> Self {
        let binding_map = bindings
            .into_iter()
            .map(|binding| (binding.key(), binding))
            .collect();

        Self {
            bindings: binding_map,
            errors: Errors::new(),
        }
    }

    pub fn link(mut self) -> LinkedBindings {
        self.check_for_missing();

        LinkedBindings {
            bindings: self.bindings
        }
    }

    fn add_error(&mut self, error: ChassisError) {
        self.errors.add(error)
    }

    pub fn check_for_missing(&mut self) {
        for binding in self.bindings.values() {
            for dep in binding.dependencies() {
                if !self.bindings.contains_key(dep.key()) {
                    self.errors.add(ChassisError::MissingImplementation(dep.key().clone()))
                }
            }
        }
    }
}

