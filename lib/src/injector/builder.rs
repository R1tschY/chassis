use crate::binder::Binder;
use crate::{Injector, Module};
use std::ops::Deref;

#[derive(Default)]
pub struct InjectorBuilder {
    modules: Vec<Box<dyn Module>>,
}

impl InjectorBuilder {
    pub fn module(&mut self, module: impl Module + 'static) -> &mut Self {
        self.modules.push(Box::new(module));
        self
    }

    pub fn build(&mut self) -> Injector {
        let mut binder = Binder::new();

        for module in &self.modules {
            binder.install(module.deref());
        }

        Injector::from_binder(binder)
    }
}
