use std::any::{type_name, TypeId};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::sync::Arc;

use crate::bind::annotation::BindAnnotation;
use std::ops::Deref;

// TODO: cache hash

/// Untyped binding key consisting of id of type to inject and an optional annotation.
///
/// Corresponds to definition at the injection point.
#[derive(Clone)]
pub struct Key {
    type_id: TypeId,
    type_name: &'static str,
    annotation: Option<AnnotationHolder>,
}

/// Typed version of `Key`.
#[derive(Hash, Eq, PartialEq)]
pub struct TypedKey<T: ?Sized + 'static>(Key, PhantomData<T>);

#[derive(Clone)]
struct AnnotationHolder(Arc<dyn BindAnnotation>, String);

impl Key {
    fn internal_new(
        type_id: TypeId,
        type_name: &'static str,
        annotation: Option<AnnotationHolder>,
    ) -> Self {
        Self {
            type_id,
            type_name,
            annotation,
        }
    }

    /// Create from only a type.
    #[inline]
    pub fn new<T: ?Sized + 'static>() -> Self {
        Self::internal_new(TypeId::of::<T>(), type_name::<T>(), None)
    }

    /// Create from type and annotation.
    pub fn new_with_annotation<T: ?Sized + 'static, U: BindAnnotation>(annotation: U) -> Self {
        Self::internal_new(
            TypeId::of::<T>(),
            type_name::<T>(),
            Some(AnnotationHolder::new(annotation)),
        )
    }

    /// Create from type and annotation.
    pub fn with_annotation<U: BindAnnotation>(&self, annotation: U) -> Self {
        Self::internal_new(
            self.type_id,
            self.type_name,
            Some(AnnotationHolder::new(annotation)),
        )
    }

    /// Type id
    pub fn type_id(&self) -> TypeId {
        self.type_id
    }

    /// Type name from `std::any::type_name`
    pub fn type_name(&self) -> &'static str {
        self.type_name
    }

    /// Type id of annotation type
    pub fn annotation_type_id(&self) -> Option<TypeId> {
        self.annotation.as_ref().map(|a| a.0.type_id())
    }

    /// annotation value formatted using `Debug` trait
    pub fn annotation_debug(&self) -> Option<&str> {
        self.annotation.as_ref().map(|a| &a.1 as &str)
    }
}

impl Hash for Key {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.type_id.hash(state);
        self.annotation.hash(state);
    }
}

impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        self.type_id == other.type_id && self.annotation == other.annotation
    }
}

impl Eq for Key {}

impl fmt::Debug for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(&format!("Key<{}>", self.type_name))
            .field(&self.annotation)
            .finish()
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

impl<T: ?Sized + 'static> Default for TypedKey<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: ?Sized + 'static> fmt::Debug for TypedKey<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(&format!("TypedKey<{}>", self.type_name()))
            .field(&self.annotation_debug())
            .finish()
    }
}

impl<T: ?Sized + 'static> Clone for TypedKey<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), PhantomData)
    }
}

impl<T: ?Sized + 'static> Deref for TypedKey<T> {
    type Target = Key;

    fn deref(&self) -> &Self::Target {
        &self.0
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
        self.0.type_id() == other.0.type_id() && self.1 == other.1
    }
}

impl fmt::Debug for AnnotationHolder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.1)
    }
}
