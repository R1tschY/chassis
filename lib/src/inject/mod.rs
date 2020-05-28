use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;

use crate::bind::binding::Binding;
use crate::inject::builder::InjectorBuilder;
use crate::resolve::ResolveFrom;
use crate::{Binder, Key, Module, Provider};
use crate::key::TypedKey;

pub mod builder;
pub mod context;

/*thread_local!(
    static CONTEXT: RefCell<InjectorContext> = RefCell::new(InjectorContext::new())
);*/

/// Holds factories of all registered types.
pub struct Injector {
    bindings: HashMap<Key, Binding>,
}

impl Injector {
    /// for tests only
    pub(crate) fn from_binder(binder: Binder) -> Self {
        Self {
            bindings: binder.link().bindings(),
        }
    }

    pub fn builder() -> InjectorBuilder {
        InjectorBuilder::default()
    }

    pub fn from_module(module: impl Module + 'static) -> Self {
        Self::builder().module(module).build()
    }

    #[inline]
    pub fn contains_type<T: ?Sized + 'static>(&self) -> bool {
        self.contains_untyped_key(Key::new::<T>().into())
    }

    #[inline]
    pub fn contains<T: ?Sized + 'static>(&self, key: TypedKey<T>) -> bool {
        self.contains_untyped_key(key.into())
    }

    pub fn contains_untyped_key(&self, key: Key) -> bool {
        self.bindings.contains_key(&key)
    }

    pub fn resolve_type<T: ?Sized + 'static>(&self) -> Option<Arc<T>> {
        self.resolve_any(Key::new::<T>().into())
            .map(|any| *any.downcast::<Arc<T>>().unwrap())
    }

    pub fn resolve<T: ?Sized + 'static>(&self, key: TypedKey<T>) -> Option<Arc<T>> {
        self.resolve_any(key.into())
            .map(|any| *any.downcast::<Arc<T>>().unwrap())
    }

    fn resolve_any(&self, key: Key) -> Option<Box<dyn Any>> {
        self.bindings
            .get(&key)
            .map(|binding| binding.factory().load(self))
    }

    /// Only use in the context of tooling!
    pub fn get_binding(&self, key: Key) -> Option<&Binding> {
        self.bindings.get(&key)
    }

    #[inline]
    pub fn resolve_to<T: ResolveFrom>(&self) -> T {
        T::resolve_from(self)
    }

    // #[inline]
    // pub fn to_provider<T: ?Sized + 'static>(&self) -> ProviderPtr<T> {
    //     assert!(self.contains::<T>());
    //     ProviderPtr::new(self)
    // }
}

// TODO: check if T is in Injector before creating a Provider
impl<'a, T: ?Sized + 'static> Provider<T> for &Injector {
    fn get(&self) -> Arc<T> {
        self.resolve_type::<T>().unwrap()
    }

    fn try_get(&self) -> Option<Arc<T>> {
        self.resolve_type::<T>()
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;
    use crate::AnonymousModule;

    trait Interface1: Debug {
        fn do_something(&self);
    }

    #[derive(Eq, PartialEq, Debug)]
    struct Impl1();

    impl Interface1 for Impl1 {
        fn do_something(&self) {
            // no nothing
        }
    }

    #[derive(Eq, PartialEq, Debug)]
    struct Impl2();

    /*
    #[test]
    fn test_resolve_existing_struct() {
        let locator = Injector::from_module(AnonymousModule::new(|mut binder| {
            binder.bind::<Impl1>().to_instance(Impl1());
        }));

        assert_eq!(Some(Arc::new(Impl1())), locator.resolve::<Impl1>());
        assert_matches!(locator.resolve::<dyn Interface1>(), None);
    }

    #[test]
    fn test_resolve_existing_interface() {
        let locator = Injector::from_module(AnonymousModule::new(|mut binder| {
            binder
                .bind::<dyn Interface1>()
                .to_arc_instance(Arc::new(Impl1()));
        }));

        assert_matches!(locator.resolve::<dyn Interface1>(), Some(_));
        assert_eq!(None, locator.resolve::<Impl1>());
    }

    #[test]
    fn test_resolve_nonexisting() {
        let locator = Injector::from_binder(Binder::new());

        assert_matches!(locator.resolve::<dyn Interface1>(), None);
        assert_eq!(None, locator.resolve::<Impl1>());
    }*/

    /*    #[test]
    fn it_works() {
        //let x: *const dyn Interface1 = std::ptr::null();
        let y: Trait<dyn Interface1> = Trait::new();
        let x: &dyn AnyTrait = &y;
        println!("{:?}", y.trait_type_id());
        println!("{:?}", x.trait_type_id());
        println!("{:?}", TypeId::of::<dyn Interface1>());
        println!("{:?}", TypeId::of::<dyn Interface1>());

        let x: Box<Impl1> = Box::new(Impl1());
        let y: Box<Interface1> = x;

        assert_eq!(2 + 2, 5);
    }*/
}
