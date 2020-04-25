#[cfg(test)]
#[macro_use]
extern crate assert_matches;
#[allow(unused_imports)]
#[macro_use]
extern crate chassis_proc_macros;

use std::sync::Arc;

#[doc(hidden)]
pub use chassis_proc_macros::*;

pub use crate::binder::Binder;
pub use crate::factory::CreatingFactory;
pub use crate::factory::ExistingFactory;
pub(crate) use crate::factory::{AnyFactory, AnyFactoryRef};
pub use crate::injector::Injector;
pub use crate::module::Module;
pub use crate::scope::Scope;

mod injector;

mod binder;
mod errors;
mod factory;
mod module;
mod resolve;
mod scope;

/// JSR-330-like Provider interface
///
/// https://javax-inject.github.io/javax-inject/api/javax/inject/Provider.html
pub trait Provider<T: ?Sized + 'static> {
    fn get(&self) -> Arc<T>;
}

pub struct ProviderPtr<T: ?Sized + 'static>(Box<dyn Provider<T>>);

impl<T: ?Sized + 'static> ProviderPtr<T> {
    pub fn new(provider: impl Provider<T> + 'static) -> Self {
        Self(Box::new(provider))
    }
}

impl<T: ?Sized + 'static> std::ops::Deref for ProviderPtr<T> {
    type Target = dyn Provider<T> + 'static;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}
