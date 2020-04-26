use std::sync::Arc;

use crate::factory::AnyFactoryRef;
use crate::{Injector, Key, Provider};

pub struct RecordedBinding {
    key: Key,
    data: Box<dyn AnyRecordedBindingData>,
}

trait AnyRecordedBindingData {
    fn link(&self) -> AnyFactoryRef;
}

pub enum BindingData<T> {
    Instance(Arc<T>),
    Factory(Arc<dyn Fn(&Injector) -> T>),
    Provider(Arc<dyn Provider<T>>),
}

pub struct LinkedBinding {
    key: Key,
    data: Box<dyn AnyRecordedBindingData>,
    factory: AnyFactoryRef,
}
