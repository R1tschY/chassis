use crate::service_locator::ServiceLocator;
use std::sync::Arc;
use std::marker::PhantomData;


pub trait Loader<T: ?Sized + 'static> {
    fn load(&self, service_locator: &ServiceLocator) -> Arc<T>;

    fn into_trait_loader<Trait: ?Sized + 'static>(self) -> AsTrait<Trait, T, Self>
        where Self: Sized, T: Sized
    {
        AsTrait(self, PhantomData, PhantomData)
    }
}

/*impl<T: ?Sized + 'static> dyn Loader<T> {

    fn into_trait_loader<Trait: ?Sized + 'static>(self) -> AsTrait<Trait, Self, T> {
        AsTrait(self, PhantomData, PhantomData)
    }

}*/

pub struct ExistingLoader<T: ?Sized + 'static>(pub Arc<T>);

impl<T: ?Sized + 'static> Loader<T> for ExistingLoader<T> {
    fn load(&self, service_locator: &ServiceLocator) -> Arc<T> {
        Arc::clone(&self.0)
    }
}

pub struct FactoryLoader<T: 'static>(pub Box<dyn Fn(&ServiceLocator) -> T>);

impl<T: 'static> Loader<T> for FactoryLoader<T> {
    fn load(&self, service_locator: &ServiceLocator) -> Arc<T> {
        Arc::new(self.0(service_locator))
    }
}

/*pub struct AsTrait<T: ?Sized + 'static, U: T + 'static, L: Loader<U>>(L, PhantomData<T>);

impl<T: ?Sized + 'static, U, L: Loader<U>> Loader<T> for AsTrait<T, U, L> {
    fn load(&self, service_locator: &ServiceLocator) -> Arc<T> {
        self.0.load(service_locator).into()
    }
}*/

pub struct AsTrait<T: ?Sized + 'static, U: 'static, L: Loader<U>>(L, PhantomData<U>, PhantomData<T>);

impl<T: ?Sized + 'static, U, L: Loader<U>> AsTrait<T, U, L> {
    pub fn new(loader: L) -> Self {
        Self(loader, PhantomData, PhantomData)
    }
}

impl<T: ?Sized + 'static, U, L: Loader<U>> Loader<U> for AsTrait<T, U, L> where Arc<U>: From<Arc<T>> {
    fn load(&self, service_locator: &ServiceLocator) -> Arc<U> {
        self.0.load(service_locator).into()
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


#[cfg(test)]
mod tests {
    use super::*;

    struct Dummy();

    trait DummyTrait { }

    impl DummyTrait for Dummy { }

    #[test]
    fn as_trait() {
        let loader = FactoryLoader(Box::new(|sf| Dummy()));
        let as_trait = loader.into_trait_loader::<DummyTrait>();

    }
}