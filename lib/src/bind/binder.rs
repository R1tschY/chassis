use std::collections::HashMap;

use crate::factory::{
    AnyFactoryRef, ArcCreatingFactory, BoxCreatingFactory, ConstantFactory, CreatingFactory,
};
use crate::{AnyFactoryImpl, Injector, Key, Module};
use std::marker::PhantomData;
use std::sync::Arc;

pub struct Binder {
    bindings: Vec<RecordedBinding>,
}

impl Binder {
    pub(crate) fn new() -> Self {
        Self {
            bindings: Vec::new(),
        }
    }

    pub fn bind<T: ?Sized + 'static>(&mut self) -> BindingBuilder<T> {
        let pos = self.bind_any(RecordedBinding::untargeted(Key::for_type::<T>()));
        BindingBuilder::new(self, pos)
    }

    fn bind_any(&mut self, binding: RecordedBinding) -> usize {
        self.bindings.push(binding);
        self.bindings.len() - 1
    }

    /// Install a Module
    #[inline]
    pub fn install(&mut self, module: &dyn Module) {
        module.configure(self)
    }

    pub(crate) fn build_bindings(self) -> HashMap<Key, AnyFactoryRef> {
        self.bindings
            .into_iter()
            .map(|binding| {
                (
                    binding.key,
                    binding.target.expect("Implementation is missing"),
                )
            })
            .collect()
    }
}

pub struct BindingBuilder<'a, T: ?Sized + 'static> {
    binder: &'a mut Binder,
    pos: usize,
    key: PhantomData<T>,
}

impl<'a, T: 'static> BindingBuilder<'a, T> {
    pub fn to_factory<U>(&mut self, factory: U)
    where
        U: Fn(&Injector) -> T + 'static,
    {
        self.set_target(Arc::new(AnyFactoryImpl::new(CreatingFactory(factory))))
    }

    pub fn to_instance(&mut self, instance: T) {
        self.set_target(Arc::new(AnyFactoryImpl::new(ConstantFactory(Arc::new(
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

    pub fn to_arc_instance(&mut self, instance: Arc<T>) {
        self.set_target(Arc::new(AnyFactoryImpl::new(ConstantFactory(instance))));
    }

    fn set_target(&mut self, factory: AnyFactoryRef) {
        self.binder.bindings[self.pos].set_target(factory);
    }

    pub fn to_arc_factory<U>(&mut self, factory: U)
    where
        U: Fn(&Injector) -> Arc<T> + 'static,
    {
        self.set_target(Arc::new(AnyFactoryImpl::new(ArcCreatingFactory(factory))))
    }

    pub fn to_box_factory<U>(&mut self, factory: U)
    where
        U: Fn(&Injector) -> Box<T> + 'static,
    {
        self.set_target(Arc::new(AnyFactoryImpl::new(BoxCreatingFactory(factory))))
    }
}

pub(crate) struct RecordedBinding {
    key: Key,
    target: Option<AnyFactoryRef>,
}

impl RecordedBinding {
    pub fn untargeted(key: Key) -> Self {
        Self { key, target: None }
    }

    pub fn set_target(&mut self, factory: AnyFactoryRef) {
        self.require_untargeted();
        self.target = Some(factory);
    }

    pub fn require_untargeted(&self) {
        if self.target.is_some() {
            panic!("Implementation is set multiple times");
        }
    }
}
