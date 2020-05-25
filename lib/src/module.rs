use crate::Binder;

pub trait Module {
    fn configure(&self, binder: &mut Binder);
}

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
