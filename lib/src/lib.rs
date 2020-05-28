#[cfg(test)]
#[macro_use]
extern crate assert_matches;
#[allow(unused_imports)]
#[macro_use]
extern crate chassis_proc_macros;

#[doc(hidden)]
pub use chassis_proc_macros::*;

pub use crate::bind::binder::Binder;
pub use crate::config::module::{AnonymousModule, Module};
pub(crate) use crate::factory::{AnyFactory, AnyFactoryImpl, AnyFactoryRef};
pub use crate::helper::*;
pub use crate::inject::Injector;
pub use crate::key::Key;
pub use crate::provider::{Provider, ProviderPtr};
pub use crate::scope::Scope;
pub use crate::bind::annotation::{BindAnnotation, Named};

mod bind;
mod config;
mod inject;

mod errors;
mod factory;
mod helper;
mod key;
mod provider;
mod resolve;
mod scope;

#[doc(hidden)]
pub mod _internal {
    pub use crate::config::dependency::Dependency;
}

pub mod meta {
    pub use crate::bind::binding::Binding;
    pub use crate::config::injection_point::InjectionPoint;
}
