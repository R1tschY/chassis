use std::marker::PhantomData;
use std::ops::Deref;
use std::process::Command;
use std::sync::Arc;

use crate::bind::binding::Binding;
use crate::bind::linker::{LinkedBindings, Linker};
use crate::config::injection_point::InjectionPoint;
use crate::factory::ConstantFactory;
use crate::{AnyFactoryImpl, AnyFactoryRef, BindAnnotation, Key, Module};

pub struct Binder {
    ready: Vec<Binding>,
    recorded: Vec<RecordedBinding>,
}

impl Binder {
    pub(crate) fn new() -> Self {
        Self {
            ready: Vec::new(),
            recorded: Vec::new(),
        }
    }

    pub fn bind<T: ?Sized + 'static>(&mut self) -> BindingBuilder<T> {
        let pos = self.bind_any(RecordedBinding::new::<T>());
        BindingBuilder::new(self, pos)
    }

    fn bind_any(&mut self, binding: RecordedBinding) -> usize {
        self.recorded.push(binding);
        self.recorded.len() - 1
    }

    /// TODO: deprecated: refactor to use Binder interface
    pub fn use_binding(&mut self, binding: Binding) {
        self.ready.push(binding);
    }

    /// Install a Module
    #[inline]
    pub fn install(&mut self, module: &dyn Module) {
        module.configure(self)
    }

    pub(crate) fn link(self) -> LinkedBindings {
        Linker::new(self.ready, self.recorded).link()
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
    pub fn to_instance(mut self, instance: T) {
        self.set_factory(Arc::new(AnyFactoryImpl::new(ConstantFactory(Arc::new(
            instance,
        )))));
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

    pub fn to_arc_instance(mut self, instance: Arc<T>) {
        self.set_factory(Arc::new(AnyFactoryImpl::new(ConstantFactory(instance))));
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
