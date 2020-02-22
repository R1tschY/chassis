#[cfg(test)] #[macro_use]
extern crate assert_matches;

#[allow(unused_imports)] #[macro_use] extern crate chassis_proc_macros;
#[doc(hidden)] pub use chassis_proc_macros::*;

pub use crate::loader::ExistingLoader;
pub use crate::loader::FactoryLoader;
pub use crate::service_locator::ServiceLocator;
use std::sync::Arc;
use std::any::{Any, TypeId};


mod factory;
mod loader;
mod service_locator;
mod resolve;


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


trait Scope {
    fn get(&self, tp: TypeId) -> Option<Box<dyn Any>>;

    fn resolve<T: ?Sized + 'static>(&self) -> Option<Arc<T>> {
        self.get(TypeId::of::<T>())
            .map(|any| *any.downcast::<Arc<T>>().unwrap())
    }

    // fn resolve_to<T: ?Sized + 'static, R: ResolveTo<T>>(&self) -> R {
    //     self.get(TypeId::of::<T>())
    //         .map(|any| R::resolve(*any.downcast::<Arc<T>>().unwrap()))
    // }
}
