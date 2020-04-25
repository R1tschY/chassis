use crate::Injector;

pub trait Module {
    fn configure(&self, sl: &mut Injector);
}
