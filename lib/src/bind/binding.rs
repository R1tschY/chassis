use crate::config::injection_point::InjectionPoint;
use crate::factory::{ArcCreatingFactory, BoxCreatingFactory, CreatingFactory};
use crate::{AnyFactoryImpl, AnyFactoryRef, Injector, Key};
use std::sync::Arc;
use crate::config::dependency::Dependency;

pub struct Binding {
    factory: AnyFactoryRef,
    injection_point: InjectionPoint,
    key: Key,
}

impl Binding {
    pub fn from_arc_factory<U, T>(factory: U, injection_point: InjectionPoint) -> Self
    where
        U: Fn(&Injector) -> Arc<T> + 'static,
        T: ?Sized + 'static,
    {
        Self {
            factory: Arc::new(AnyFactoryImpl::new(ArcCreatingFactory(factory))),
            injection_point,
            key: Key::for_type::<T>(),
        }
    }

    pub fn from_box_factory<U, T>(factory: U, injection_point: InjectionPoint) -> Self
    where
        U: Fn(&Injector) -> Box<T> + 'static,
        T: ?Sized + 'static,
    {
        Self {
            factory: Arc::new(AnyFactoryImpl::new(BoxCreatingFactory(factory))),
            injection_point,
            key: Key::for_type::<T>(),
        }
    }

    pub fn from_factory<U, T>(factory: U, injection_point: InjectionPoint) -> Self
    where
        U: Fn(&Injector) -> T + 'static,
        T: 'static,
    {
        Self {
            factory: Arc::new(AnyFactoryImpl::new(CreatingFactory(factory))),
            injection_point,
            key: Key::for_type::<T>(),
        }
    }

    pub fn key(&self) -> Key {
        self.key.clone()
    }

    pub(crate) fn factory(&self) -> AnyFactoryRef {
        self.factory.clone()
    }

    pub fn injection_point(&self) -> &InjectionPoint {
        &self.injection_point
    }

    pub fn dependencies(&self) -> &[Dependency] {
        self.injection_point.dependencies()
    }
}
