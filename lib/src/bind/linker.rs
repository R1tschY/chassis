use crate::bind::binder::RecordedBinding;
use crate::errors::{ChassisError, Errors};
use crate::meta::Binding;
use crate::Key;
use std::collections::HashMap;

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
    pub(crate) fn new(mut bindings: Vec<Binding>, recorded: Vec<RecordedBinding>) -> Self {
        let mut recorded_bindings: Vec<Binding> =
            recorded.into_iter().map(|binding| binding.into()).collect();
        bindings.append(&mut recorded_bindings);
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
            bindings: self.bindings,
        }
    }

    pub fn check_for_missing(&mut self) {
        for binding in self.bindings.values() {
            for dep in binding.dependencies() {
                if !self.bindings.contains_key(dep.key()) {
                    self.errors
                        .add(ChassisError::MissingImplementation(dep.key().clone()))
                }
            }
        }
    }
}
