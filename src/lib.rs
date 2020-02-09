use std::marker::PhantomData;
use std::default::Default;
use std::any::{TypeId, Any};
use std::collections::HashMap;
use std::sync::Arc;
use std::cell::{Cell, RefCell};

#[cfg(test)] #[macro_use]
extern crate assert_matches;

pub struct Trait<T: ?Sized + 'static>(PhantomData<*const T>);

impl<T: ?Sized + 'static> Trait<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

trait AnyTrait {
    fn trait_type_id(&self) -> TypeId;
}

impl<T: ?Sized + 'static> AnyTrait for Trait<T> {
    fn trait_type_id(&self) -> TypeId {
        TypeId::of::<T>()
    }
}

pub trait Loader<T: ?Sized + 'static> {
    fn load(&self, service_locator: &ServiceLocator) -> &Arc<T>;
}

pub struct ExistingLoader<T: ?Sized + 'static>(Arc<T>);

impl<T: ?Sized + 'static> Loader<T> for ExistingLoader<T> {
    fn load(&self, service_locator: &ServiceLocator) -> &Arc<T> {
        &self.0
    }
}

/*pub struct SingletonLoader<T: ?Sized + 'static> {
    loader: Box<dyn Loader<T>>,
    cache: RefCell<Option<Arc<T>>>
}

impl<T: ?Sized + 'static> Loader<T> for SingletonLoader<T> {
    fn load(&self, service_locator: &ServiceLocator) -> &Arc<T> {
        {
            let mut instance = self.cache.borrow_mut();
            if instance.is_none() {
                instance = Some(Arc::clone(self.loader.load(service_locator)));
            }
        }
        &self.cache.borrow().unwrap()
    }
}*/

trait AnyLoader {
    fn load(&self, service_locator: &ServiceLocator) -> &dyn Any;
}

struct AnyLoaderFn<T: ?Sized + 'static>(Arc<dyn Loader<T>>);

impl<T: ?Sized + 'static> AnyLoader for AnyLoaderFn<T> {
    fn load(&self, service_locator: &ServiceLocator) -> &dyn Any {
        self.0.load(service_locator)
    }
}

pub struct ServiceLocator {
    bindings: HashMap<TypeId, Box<dyn AnyLoader>>
}

impl ServiceLocator {
    pub fn new() -> Self {
        Self { bindings: HashMap::new() }
    }

    pub fn resolve<T: ?Sized + 'static>(&self) -> Option<Arc<T>> {
        self.resolve_any(&TypeId::of::<T>())
            .map(|any| Arc::clone(any.downcast_ref::<Arc<T>>().unwrap()))
    }

    fn resolve_any(&self, id: &TypeId) -> Option<&dyn Any> {
        self.bindings.get(id).map(|loader| loader.load(self))
    }

    pub fn register<T: ?Sized + 'static, U: Loader<T> + 'static>(&mut self, loader: U) {
        self.register_any(TypeId::of::<T>(), Box::new(AnyLoaderFn(Arc::new(loader))));
    }

    fn register_any(&mut self, id: TypeId, loader: Box<dyn AnyLoader>) {
        self.bindings.insert(id, loader);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::any::{Any, TypeId};
    use std::fmt::Debug;

    trait Interface1 : Debug {
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
        let mut locator = ServiceLocator::new();
        locator.register(ExistingLoader(Arc::new(Impl1())));
        assert_eq!(Some(Arc::new(Impl1())), locator.resolve::<Impl1>());
        assert_matches!(locator.resolve::<dyn Interface1>(), None);
    }

    #[test]
    fn test_resolve_existing_interface() {
        let mut locator = ServiceLocator::new();
        locator.register(ExistingLoader::<dyn Interface1>(Arc::new(Impl1())));
        assert_matches!(locator.resolve::<dyn Interface1>(), Some(_));
        assert_eq!(None, locator.resolve::<Impl1>());
    }

    #[test]
    fn test_resolve_nonexisting() {
        let mut locator = ServiceLocator::new();
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
