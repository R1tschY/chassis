use std::collections::HashMap;

use crate::factory::{AnyFactoryRef, ConstantFactory, CreatingFactory};
use crate::{AnyFactoryImpl, Injector, Key, Module};
use std::sync::Arc;

pub struct Binder {
    bindings: HashMap<Key, AnyFactoryRef>,
}

impl Binder {
    pub(crate) fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }

    pub fn bind_factory<T, U>(&mut self, factory: U)
    where
        T: 'static,
        U: Fn(&Injector) -> T + 'static,
    {
        self.bind_any(
            Key::for_type::<T>(),
            Arc::new(AnyFactoryImpl::new(CreatingFactory::new(factory))),
        );
    }

    pub fn bind_instance<T: 'static>(&mut self, instance: T) {
        self.bind_any(
            Key::for_type::<T>(),
            Arc::new(AnyFactoryImpl::new(ConstantFactory(Arc::new(instance)))),
        );
    }

    pub fn bind_arc_instance<T: ?Sized + 'static>(&mut self, instance: Arc<T>) {
        self.bind_any(
            Key::for_type::<T>(),
            Arc::new(AnyFactoryImpl::new(ConstantFactory(instance))),
        );
    }

    fn bind_any(&mut self, key: Key, loader: AnyFactoryRef) {
        self.bindings.insert(key, loader);
    }

    /// Install a Module
    #[inline]
    pub fn install(&mut self, module: &dyn Module) {
        module.configure(self)
    }

    pub(crate) fn build_bindings(self) -> HashMap<Key, AnyFactoryRef> {
        self.bindings
    }
}
