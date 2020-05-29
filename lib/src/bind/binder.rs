use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use crate::bind::binding::Binding;
use crate::bind::linker::{LinkedBindings, Linker};
use crate::config::injection_point::InjectionPoint;
use crate::factory::{
    to_any_factory, ArcCreatingFactory, BoxCreatingFactory, ConstantFactory, CreatingFactory,
};
use crate::{AnyFactoryRef, BindAnnotation, Injector, Key, Module};

pub struct Binder {
    recorded: Vec<RecordedBinding>,
}

impl Binder {
    pub(crate) fn new() -> Self {
        Self {
            recorded: Vec::new(),
        }
    }

    pub fn bind<T: ?Sized + 'static>(&mut self) -> AnnotatedBindingBuilder<T> {
        let pos = self.bind_any(RecordedBinding::new::<T>());
        AnnotatedBindingBuilder::new(self, pos)
    }

    fn bind_any(&mut self, binding: RecordedBinding) -> usize {
        self.recorded.push(binding);
        self.recorded.len() - 1
    }

    /// Install a Module
    #[inline]
    pub fn install(&mut self, module: &dyn Module) {
        module.configure(self)
    }

    pub(crate) fn link(self) -> LinkedBindings {
        Linker::new(self.recorded).link()
    }
}

pub(crate) struct RecordedBinding {
    factory: Option<AnyFactoryRef>,
    injection_point: Option<InjectionPoint>,
    key: Key,
}

impl RecordedBinding {
    pub fn new<T: ?Sized + 'static>() -> Self {
        Self {
            factory: None,
            injection_point: None,
            key: Key::new::<T>(),
        }
    }
}

impl From<RecordedBinding> for Binding {
    fn from(other: RecordedBinding) -> Self {
        Binding::new(
            other.factory.expect("Untargetted binding found"),
            other.injection_point,
            other.key,
        )
    }
}

pub struct BindingBuilder<'a, T: ?Sized + 'static> {
    binder: &'a mut Binder,
    pos: usize,
    key: PhantomData<T>,
}

impl<'a, T: 'static> BindingBuilder<'a, T> {
    #[allow(clippy::wrong_self_convention)]
    pub fn to_instance(&mut self, instance: T) {
        self.set_factory(to_any_factory(ConstantFactory(Arc::new(instance))));
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn to_factory<U>(&mut self, factory: U, injection_point: InjectionPoint)
    where
        U: Fn(&Injector) -> T + 'static,
    {
        self.to_any_factory(to_any_factory(CreatingFactory(factory)), injection_point)
    }
}

impl<'a, T: ?Sized + 'static> BindingBuilder<'a, T> {
    fn new(binder: &'a mut Binder, pos: usize) -> Self {
        Self {
            binder,
            pos,
            key: PhantomData,
        }
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn to_arc_factory<U>(&mut self, factory: U, injection_point: InjectionPoint)
    where
        U: Fn(&Injector) -> Arc<T> + 'static,
    {
        self.to_any_factory(to_any_factory(ArcCreatingFactory(factory)), injection_point)
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn to_box_factory<U>(&mut self, factory: U, injection_point: InjectionPoint)
    where
        U: Fn(&Injector) -> Box<T> + 'static,
    {
        self.to_any_factory(to_any_factory(BoxCreatingFactory(factory)), injection_point)
    }

    #[allow(clippy::wrong_self_convention)]
    fn to_any_factory(&mut self, factory: AnyFactoryRef, injection_point: InjectionPoint) {
        self.set_factory(factory);
        self.set_injection_point(injection_point);
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn to_arc_instance(&mut self, instance: Arc<T>) {
        self.set_factory(to_any_factory(ConstantFactory(instance)));
    }

    fn set_injection_point(&mut self, injection_point: InjectionPoint) {
        self.binder.recorded[self.pos].injection_point = Some(injection_point);
    }

    fn set_factory(&mut self, factory: AnyFactoryRef) {
        self.binder.recorded[self.pos].factory = Some(factory);
    }

    fn set_annotation<U: BindAnnotation>(&mut self, annotation: U) {
        let key = &mut self.binder.recorded[self.pos].key;
        *key = key.clone().with_annotation(annotation);
    }
}

pub struct AnnotatedBindingBuilder<'a, T: ?Sized + 'static>(BindingBuilder<'a, T>);

impl<'a, T: ?Sized + 'static> AnnotatedBindingBuilder<'a, T> {
    fn new(binder: &'a mut Binder, pos: usize) -> Self {
        Self(BindingBuilder::new(binder, pos))
    }

    pub fn annotated_with<U: BindAnnotation>(&mut self, annotation: U) -> &mut Self {
        self.0.set_annotation(annotation);
        self
    }
}

impl<'a, T: ?Sized + 'static> Deref for AnnotatedBindingBuilder<'a, T> {
    type Target = BindingBuilder<'a, T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, T: ?Sized + 'static> DerefMut for AnnotatedBindingBuilder<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
