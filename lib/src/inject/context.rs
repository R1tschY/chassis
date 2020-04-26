use crate::Key;

/// Used for detecting cyclic dependencies
pub struct ConstructorContext {
    constructing: bool,
}

pub struct InjectorContext {
    dependency_stack: Vec<Key>,
}
