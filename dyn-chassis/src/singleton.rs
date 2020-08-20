use crate::factory::{AnyFactory, Product};
use crate::{AnyFactoryRef, Injector, Key, Scope};
use std::fmt;
use std::ops::Deref;
use std::sync::{Arc, Mutex};

pub(crate) struct SingletonScope;

impl Scope for SingletonScope {
    fn scope(&self, _key: &Key, unscoped: AnyFactoryRef) -> AnyFactoryRef {
        Arc::new(SingletonFactory {
            unscoped,
            maybe_contructed: Mutex::new(None),
        })
    }
}

impl fmt::Debug for SingletonScope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("chassis::SINGLETON")
    }
}

struct SingletonFactory {
    unscoped: AnyFactoryRef,
    maybe_contructed: Mutex<Option<Product>>, // TODO: Use RWLOCK?
}

impl AnyFactory for SingletonFactory {
    fn load(&self, injector: &Injector) -> Product {
        let mut maybe_constructed = self
            .maybe_contructed
            .lock()
            .expect("Poisoned singleton mutex");
        if let Some(product) = maybe_constructed.deref() {
            product.clone()
        } else {
            let product = self.unscoped.load(injector);
            *maybe_constructed = Some(product.clone());
            product
        }
    }
}
