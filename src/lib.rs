#[cfg(test)] #[macro_use]
extern crate assert_matches;

#[allow(unused_imports)] #[macro_use] extern crate chassis_proc_macros;
#[doc(hidden)] pub use chassis_proc_macros::*;

pub use crate::loader::ExistingLoader;
pub use crate::loader::FactoryLoader;
pub use crate::service_locator::ServiceLocator;
pub use crate::module::Module;
pub use crate::scope::Scope;
use std::sync::Arc;
use std::any::{Any, TypeId};


mod factory;
mod loader;
mod service_locator;
mod resolve;
mod module;
mod scope;
mod errors;


/// JSR-330-like Provider interface
///
/// https://javax-inject.github.io/javax-inject/api/javax/inject/Provider.html
pub trait Provider<T: ?Sized + 'static> {
    fn get(&self) -> Arc<T>;
}

struct ProviderPtr<T: ?Sized + 'static>(Box<dyn Provider<T>>);

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
