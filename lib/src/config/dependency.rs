use crate::Key;

/// A variable that can be resolved by an injector.
pub struct Dependency {
    key: Key,
    optional: bool,
    parameter_index: i8,
}

impl Dependency {
    pub fn new(key: Key, optional: bool, parameter_index: i8) -> Self {
        Self {
            key,
            optional,
            parameter_index,
        }
    }
}
