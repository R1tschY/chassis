use std::sync::Arc;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use crate::loader::Loader;
use crate::{Provider, ProviderPtr, Module};
use crate::resolve::{ResolveFrom};

trait AnyLoader {
    fn load(&self, service_locator: &ServiceLocator) -> Box<dyn Any>;
}

struct AnyLoaderRef<T: ?Sized + 'static>(Arc<dyn Loader<T>>);

impl<T: ?Sized + 'static> AnyLoader for AnyLoaderRef<T> {
    fn load(&self, service_locator: &ServiceLocator) -> Box<dyn Any> {
        Box::new(self.0.load(service_locator))
    }
}

pub struct ServiceLocator {
    bindings: HashMap<TypeId, Box<dyn AnyLoader>>
}

impl ServiceLocator {
    pub fn new() -> Self {
        Self { bindings: HashMap::new() }
    }

    #[inline]
    pub fn contains<T: ?Sized + 'static>(&self) -> bool {
        self.contains_loader(TypeId::of::<T>())
    }

    pub fn resolve<T: ?Sized + 'static>(&self) -> Option<Arc<T>> {
        self.resolve_any(TypeId::of::<T>())
            .map(|any| *any.downcast::<Arc<T>>().unwrap())
    }

    #[inline]
    pub fn resolve_to<T: ResolveFrom>(&self) -> T {
        T::resolve_from(self)
    }

    fn contains_loader(&self, id: TypeId) -> bool {
        self.bindings.contains_key(&id)
    }

    fn resolve_any(&self, id: TypeId) -> Option<Box<dyn Any>> {
        self.bindings.get(&id).map(|loader| loader.load(self))
    }

    pub fn register<T: ?Sized + 'static, U: Loader<T> + 'static>(&mut self, loader: U) {
        self.register_any(TypeId::of::<T>(), Box::new(AnyLoaderRef(Arc::new(loader))));
    }

    fn register_any(&mut self, id: TypeId, loader: Box<dyn AnyLoader>) {
        self.bindings.insert(id, loader);
    }

    #[inline]
    pub fn install(&mut self, module: &impl Module) {
        module.configure(self)
    }

    // #[inline]
    // pub fn to_provider<T: ?Sized + 'static>(&self) -> ProviderPtr<T> {
    //     assert!(self.contains::<T>());
    //     ProviderPtr::new(self)
    // }
}

// TODO: check if T is in ServiceLocator before creating a Provider
impl<'a, T: ?Sized + 'static> Provider<T> for &ServiceLocator {
    fn get(&self) -> Arc<T> {
        self.resolve::<T>().unwrap()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Debug;
    use crate::loader::ExistingLoader;

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
        let locator = ServiceLocator::new();
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
