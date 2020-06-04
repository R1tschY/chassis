use crate::{AnyFactoryRef, Key};
use std::fmt::Debug;

pub trait Scope: Debug {
    fn scope(&self, key: &Key, unscoped: AnyFactoryRef) -> AnyFactoryRef;
}

pub(crate) type ScopePtr = &'static dyn Scope;
