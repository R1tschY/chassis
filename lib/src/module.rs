use crate::binder::Binder;

pub trait Module {
    fn configure(&self, binder: &mut Binder);
}
