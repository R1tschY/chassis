use crate::config::dependency::Dependency;
use crate::config::injection_point::InjectionPoint;
use crate::{AnyFactoryRef, BindAnnotation, Key};

const NO_DEPENDENCIES: &[Dependency] = &[];

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum BindingType {
    Factory,
    Instance,
}

/// A binding
pub struct Binding {
    factory: AnyFactoryRef,
    injection_point: Option<InjectionPoint>,
    binding_type: BindingType,
    key: Key,
}

impl Binding {
    pub(crate) fn new(
        factory: AnyFactoryRef,
        injection_point: Option<InjectionPoint>,
        binding_type: BindingType,
        key: Key,
    ) -> Self {
        Self {
            factory,
            injection_point,
            binding_type,
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

    /// type of binding used
    pub fn binding_type(&self) -> BindingType {
        self.binding_type.clone()
    }

    /// Dependencies needed to resolve type
    pub fn dependencies(&self) -> &[Dependency] {
        self.injection_point
            .as_ref()
            .map_or(NO_DEPENDENCIES, |ip| ip.dependencies())
    }
}
