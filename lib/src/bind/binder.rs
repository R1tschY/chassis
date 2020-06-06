use std::marker::PhantomData;
use std::sync::Arc;

use crate::bind::binding::{Binding, BindingType};
use crate::bind::linker::{LinkedBindings, Linker};
use crate::config::injection_point::InjectionPoint;
use crate::factory::{
    to_any_factory, ArcCreatingFactory, BoxCreatingFactory, ConstantFactory, CreatingFactory,
};
use crate::scope::ScopePtr;
use crate::{AnyFactoryRef, BindAnnotation, ChassisResult, Injector, Key, Module};

#[cfg(nightly_unsize)]
use std::marker::Unsize;

#[cfg(nightly_unsize)]
use crate::factory::ConverterBuilder;
use std::any::type_name;

pub struct Binder {
    recorded: Vec<RecordedBinding>,
}

impl Binder {
    pub(crate) fn new() -> Self {
        Self {
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

    /// Install a Module
    #[inline]
    pub fn install(&mut self, module: &dyn Module) {
        module.configure(self)
    }

    pub(crate) fn link(self) -> ChassisResult<LinkedBindings> {
        Linker::new(self.recorded).link()
    }
}

pub(crate) struct RecordedBinding {
    factory: Option<AnyFactoryRef>,
    injection_point: Option<InjectionPoint>,
    key: Key,
    binding_type: Option<BindingType>,
    scope: Option<ScopePtr>,
}

impl RecordedBinding {
    pub fn new<T: ?Sized + 'static>() -> Self {
        Self {
            factory: None,
            injection_point: None,
            key: Key::new::<T>(),
            binding_type: None,
            scope: None,
        }
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
        self.set_factory(
            to_any_factory(ConstantFactory(Arc::new(instance))),
            BindingType::Instance,
        );
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
        self.set_factory(factory, BindingType::Factory);
        self.set_injection_point(injection_point);
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn to_arc_instance(&mut self, instance: Arc<T>) {
        self.set_factory(
            to_any_factory(ConstantFactory(instance)),
            BindingType::Instance,
        );
    }

    fn set_injection_point(&mut self, injection_point: InjectionPoint) {
        self.binder.recorded[self.pos].injection_point = Some(injection_point);
    }

    fn set_factory(&mut self, factory: AnyFactoryRef, binding_type: BindingType) {
        self.binder.recorded[self.pos].binding_type = Some(binding_type);
        self.binder.recorded[self.pos].factory = Some(factory);
    }

    pub fn annotated_with<U: BindAnnotation>(&mut self, annotation: U) -> &mut Self {
        let key = &mut self.binder.recorded[self.pos].key;
        *key = key.clone().with_annotation(annotation);
        self
    }

    /// specify scope for binding
    pub fn in_(&mut self, scope: ScopePtr) -> &mut Self {
        self.binder.recorded[self.pos].scope = Some(scope);
        self
    }

    /// create linked binding to a other type.
    ///
    /// The other type have to be binded.
    #[cfg(nightly_unsize)]
    pub fn to_type<U: Unsize<T> + ?Sized + 'static>(&mut self) -> &mut Self {
        self.set_factory(
            to_any_factory(ConverterBuilder::<T, U>::new().build(|x| x)),
            BindingType::Linked,
        );
        self.set_injection_point(InjectionPoint::for_module_function(
            "BindingBuilder::to_type",
            &[Key::new::<U>()],
        ));
        self
    }
}

impl From<RecordedBinding> for Binding {
    fn from(recorded: RecordedBinding) -> Self {
        let factory = recorded.factory.expect("Untargetted binding found");
        let factory = if let Some(scope) = recorded.scope {
            scope.scope(&recorded.key, factory)
        } else {
            factory
        };
        Binding::new(
            factory,
            recorded.injection_point,
            recorded.binding_type.unwrap(),
            recorded.key,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Class;

    trait Trait1 {}
    impl Trait1 for Class {}

    #[cfg(nightly_unsize)]
    #[test]
    fn test_bind_to_type() {
        let mut binder = Binder::new();
        binder.bind::<Class>().to_instance(Class);
        binder.bind::<dyn Trait1>().to_type::<Class>();

        assert_eq!(2, binder.recorded.len());
        assert_eq!(Key::new::<dyn Trait1>(), binder.recorded[1].key);
        assert_eq!(Some(BindingType::Linked), binder.recorded[1].binding_type);
        assert!(binder.recorded[1].injection_point.is_some());

        let injection_point = binder.recorded[1].injection_point.as_ref().unwrap();
        assert_eq!("BindingBuilder::to_type", injection_point.member());
        assert_eq!(1, injection_point.dependencies().len());
        assert_eq!(
            &Key::new::<Class>(),
            injection_point.dependencies()[0].key()
        );
    }
}
