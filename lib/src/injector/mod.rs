use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;

use crate::binder::{Binder, Key};
use crate::injector::builder::InjectorBuilder;
use crate::resolve::ResolveFrom;
use crate::{AnyFactory, Provider};

pub mod builder;
pub mod context;

/// Holds factories of all registered types.
pub struct Injector {
    bindings: HashMap<Key, Box<dyn AnyFactory>>,
}

impl Injector {
    /// for tests only
    pub(crate) fn from_binder(binder: Binder) -> Self {
        Self {
            bindings: binder.build_bindings(),
        }
    }

    pub fn builder() -> InjectorBuilder {
        InjectorBuilder::default()
    }

    #[inline]
    pub fn contains<T: ?Sized + 'static>(&self) -> bool {
        self.contains_factory(Key::for_type::<T>())
    }

    fn contains_factory(&self, key: Key) -> bool {
        self.bindings.contains_key(&key)
    }

    pub fn resolve<T: ?Sized + 'static>(&self) -> Option<Arc<T>> {
        self.resolve_any(Key::for_type::<T>())
            .map(|any| *any.downcast::<Arc<T>>().unwrap())
    }

    fn resolve_any(&self, key: Key) -> Option<Box<dyn Any>> {
        self.bindings.get(&key).map(|factory| factory.load(self))
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
        self.resolve::<T>().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use crate::factory::ExistingFactory;

    use super::*;

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

    #[test]
    fn test_resolve_existing_struct() {
        let mut binder = Binder::new();
        binder.bind(ExistingFactory(Arc::new(Impl1())));
        let locator = Injector::from_binder(binder);

        assert_eq!(Some(Arc::new(Impl1())), locator.resolve::<Impl1>());
        assert_matches!(locator.resolve::<dyn Interface1>(), None);
    }

    #[test]
    fn test_resolve_existing_interface() {
        let mut binder = Binder::new();
        binder.bind(ExistingFactory::<dyn Interface1>(Arc::new(Impl1())));
        let locator = Injector::from_binder(binder);

        assert_matches!(locator.resolve::<dyn Interface1>(), Some(_));
        assert_eq!(None, locator.resolve::<Impl1>());
    }

    #[test]
    fn test_resolve_nonexisting() {
        let locator = Injector::from_binder(Binder::new());

        assert_matches!(locator.resolve::<dyn Interface1>(), None);
        assert_eq!(None, locator.resolve::<Impl1>());
    }

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
