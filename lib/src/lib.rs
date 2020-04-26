#[cfg(test)]
#[macro_use]
extern crate assert_matches;
#[allow(unused_imports)]
#[macro_use]
extern crate chassis_proc_macros;

#[doc(hidden)]
pub use chassis_proc_macros::*;

pub use crate::bind::binder::Binder;
pub(crate) use crate::factory::{AnyFactory, AnyFactoryImpl, AnyFactoryRef};
pub use crate::helper::*;
pub use crate::inject::Injector;
pub use crate::key::Key;
pub use crate::module::{AnonymousModule, Module};
pub use crate::provider::{Provider, ProviderPtr};
pub use crate::scope::Scope;

mod bind;
mod inject;

mod errors;
mod factory;
mod helper;
mod key;
mod module;
mod provider;
mod resolve;
mod scope;
