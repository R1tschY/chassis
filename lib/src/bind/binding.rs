use crate::config::dependency::Dependency;
use crate::config::injection_point::InjectionPoint;
use crate::factory::{ArcCreatingFactory, BoxCreatingFactory, CreatingFactory};
use crate::key::TypedKey;
use crate::{AnyFactoryImpl, AnyFactoryRef, BindAnnotation, Injector, Key};
use std::sync::Arc;

const NO_DEPENDENCIES: &[Dependency] = &[];

/// A binding
pub struct Binding {
    factory: AnyFactoryRef,
    injection_point: Option<InjectionPoint>,
    key: Key,
}

impl Binding {
    pub fn from_arc_factory<U, T>(factory: U, injection_point: InjectionPoint) -> Self
    where
        U: Fn(&Injector) -> Arc<T> + 'static,
        T: ?Sized + 'static,
    {
        Self::from_any_factory::<T>(
            Arc::new(AnyFactoryImpl::new(ArcCreatingFactory(factory))),
            injection_point,
        )
    }

    pub fn from_box_factory<U, T>(factory: U, injection_point: InjectionPoint) -> Self
    where
        U: Fn(&Injector) -> Box<T> + 'static,
        T: ?Sized + 'static,
    {
        Self::from_any_factory::<T>(
            Arc::new(AnyFactoryImpl::new(BoxCreatingFactory(factory))),
            injection_point,
        )
    }

    pub fn from_factory<U, T>(factory: U, injection_point: InjectionPoint) -> Self
    where
        U: Fn(&Injector) -> T + 'static,
        T: 'static,
    {
        Self::from_any_factory::<T>(
            Arc::new(AnyFactoryImpl::new(CreatingFactory(factory))),
            injection_point,
        )
    }

    fn from_any_factory<T: ?Sized + 'static>(
        factory: AnyFactoryRef,
        injection_point: InjectionPoint,
    ) -> Self {
        Self::new(factory, Some(injection_point), Key::new::<T>())
    }

    pub(crate) fn new(
        factory: AnyFactoryRef,
        injection_point: Option<InjectionPoint>,
        key: Key,
    ) -> Self {
        Self {
            factory,
            injection_point,
            key,
        }
    }

    /// Create binding overwriting existing annotation
    pub fn with_annotation<U: BindAnnotation>(self, annotation: U) -> Self {
        Self {
            key: self.key.with_annotation(annotation),
            ..self
        }
    }

    /// Key of the binding
    pub fn key(&self) -> Key {
        self.key.clone()
    }

    /// Factory to create the type described in key
    pub(crate) fn factory(&self) -> AnyFactoryRef {
        self.factory.clone()
    }

    /// Point where dependencies have to be injected to resolve type
    pub fn injection_point(&self) -> Option<&InjectionPoint> {
        self.injection_point.as_ref()
    }

    /// Dependencies needed to resolve type
    pub fn dependencies(&self) -> &[Dependency] {
        self.injection_point
            .as_ref()
            .map_or(NO_DEPENDENCIES, |ip| ip.dependencies())
    }
}
