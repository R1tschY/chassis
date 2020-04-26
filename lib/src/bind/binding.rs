use std::sync::Arc;

use crate::factory::AnyFactoryRef;
use crate::{Injector, Key, Provider};

// trait AnyBindingTarget {
//     fn link(&self) -> AnyFactoryRef;
// }

// pub(crate) enum BindingTarget<T> {
//     Instance(Arc<T>),
//     Factory(Arc<dyn Fn(&Injector) -> T>),
//     Provider(Arc<dyn Provider<T>>),
// }
//
// pub(crate) struct LinkedBinding {
//     key: Key,
//     data: Box<dyn AnyBindingTarget>,
//     factory: AnyFactoryRef,
// }
