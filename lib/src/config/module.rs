use crate::Binder;

/// A injection module
pub trait Module {
    /// Configure bindings
    fn configure(&self, binder: &mut Binder);
}

/// A module backed by a function
pub struct AnonymousModule<T: Fn(&mut Binder)>(T);

impl<T: Fn(&mut Binder)> AnonymousModule<T> {
    pub fn new(f: T) -> Self {
        Self(f)
    }
}

impl<T: Fn(&mut Binder)> Module for AnonymousModule<T> {
    fn configure(&self, binder: &mut Binder) {
        self.0(binder);
    }
}
