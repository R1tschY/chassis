use std::marker::PhantomData;
use std::default::Default;
use std::any::{TypeId, Any};
use std::collections::HashMap;
use std::sync::Arc;
use std::cell::{Cell, RefCell};

#[cfg(test)] #[macro_use]
extern crate assert_matches;

mod factory;
mod loader;
mod service_locator;

pub use crate::service_locator::ServiceLocator;
pub use crate::loader::ExistingLoader;
pub use crate::loader::FactoryLoader;


