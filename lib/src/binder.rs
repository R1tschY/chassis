use std::any::TypeId;
use std::collections::HashMap;

use crate::factory::Factory;
use crate::{AnyFactory, AnyFactoryRef, Module};

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct Key(TypeId);

impl Key {
    pub fn for_type<T: ?Sized + 'static>() -> Self {
        Self(TypeId::of::<T>())
    }
}

pub struct Binder {
    bindings: HashMap<Key, Box<dyn AnyFactory>>,
}

impl Binder {
    pub(crate) fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }

    /// Register factory
    pub fn bind<T: ?Sized + 'static, U: Factory<T> + 'static>(&mut self, factory: U) {
        self.bind_any(Key::for_type::<T>(), Box::new(AnyFactoryRef::new(factory)));
    }

    fn bind_any(&mut self, key: Key, loader: Box<dyn AnyFactory>) {
        self.bindings.insert(key, loader);
    }

    /// Install a Module
    #[inline]
    pub fn install(&mut self, module: &dyn Module) {
        module.configure(self)
    }

    pub(crate) fn build_bindings(self) -> HashMap<Key, Box<dyn AnyFactory>> {
        self.bindings
    }
}
