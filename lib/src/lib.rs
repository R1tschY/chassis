#[doc(hidden)]
pub use chassis_proc_macros::{factory, inject, module};

pub use crate::bind::annotation::{BindAnnotation, Named};
pub use crate::bind::binder::Binder;
pub use crate::config::module::{AnonymousModule, Module};
pub use crate::errors::{ChassisError, ChassisResult, Errors};
pub(crate) use crate::factory::AnyFactoryRef;
pub use crate::helper::*;
pub use crate::inject::Injector;
pub use crate::key::{Key, TypedKey};
pub use crate::provider::{Provider, ProviderPtr};
pub use crate::scope::Scope;

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
