use std::any::TypeId;
use std::fmt;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::sync::Arc;

use crate::bind::annotation::BindAnnotation;

// TODO: cache hash

/// Untyped binding key consisting of id of type to inject and an optional annotation.
///
/// Corresponds to definition at the injection point.
#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct Key(TypeId, Option<AnnotationHolder>);

/// Typed version of `Key`.
#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct TypedKey<T: ?Sized + 'static>(Key, PhantomData<T>);

#[derive(Clone)]
struct AnnotationHolder(Arc<dyn BindAnnotation>, String);

impl Key {
    /// Create from only a type.
    #[inline]
    pub fn new<T: ?Sized + 'static>() -> Self {
        Self(TypeId::of::<T>(), None)
    }

    /// Create from type and annotation.
    pub fn new_with_annotation<T: ?Sized + 'static, U: BindAnnotation>(annotation: U) -> Self {
        Self(TypeId::of::<T>(), Some(AnnotationHolder::new(annotation)))
    }

    /// Create from type and annotation.
    pub fn with_annotation<U: BindAnnotation>(&self, annotation: U) -> Self {
        Self(self.0, Some(AnnotationHolder::new(annotation)))
    }
}

impl<T: ?Sized + 'static> TypedKey<T> {
    /// Create from only a type.
    #[inline]
    pub fn new() -> Self {
        Self(Key::new::<T>(), PhantomData)
    }

    /// Create from type and annotation.
    #[inline]
    pub fn new_with_annotation<U: BindAnnotation>(annotation: U) -> Self {
        Self(Key::new_with_annotation::<T, U>(annotation), PhantomData)
    }
}

impl AnnotationHolder {
    fn new<U: BindAnnotation>(annotation: U) -> Self {
        let debug_annotation = format!("{:?}", &annotation);
        let annotation: Arc<dyn BindAnnotation> = Arc::new(annotation);
        Self(annotation, debug_annotation)
    }
}

impl<T: ?Sized + 'static> From<TypedKey<T>> for Key {
    fn from(key: TypedKey<T>) -> Self {
        key.0
    }
}

impl Hash for AnnotationHolder {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.type_id().hash(state);
        self.1.hash(state);
    }
}

impl Eq for AnnotationHolder {}

impl PartialEq for AnnotationHolder {
    fn eq(&self, other: &Self) -> bool {
        self.0.type_id() == other.0.type_id() && self.1 == self.1
    }
}

impl Debug for AnnotationHolder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.1)
    }
}
