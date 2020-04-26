use crate::Binder;

pub trait Module {
    fn configure(&self, binder: &mut Binder);
}

pub struct AnonymousModule(Box<dyn Fn(&mut Binder)>);

impl AnonymousModule {
    pub fn new(f: impl Fn(&mut Binder) + 'static) -> Self {
        Self(Box::new(f))
    }
}

impl Module for AnonymousModule {
    fn configure(&self, binder: &mut Binder) {
        self.0(binder);
    }
}
