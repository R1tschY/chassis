use std::marker::PhantomData;
use std::sync::Arc;

use crate::Injector;
use std::any::Any;

pub trait Factory<T: ?Sized + 'static> {
    fn load(&self, injector: &Injector) -> Arc<T>;

    fn into_trait_loader<Trait: ?Sized + 'static>(self) -> AsTrait<Trait, T, Self>
    where
        Self: Sized,
        T: Sized,
    {
        AsTrait(self, PhantomData, PhantomData)
    }
}

/*impl<T: ?Sized + 'static> dyn Loader<T> {

    fn into_trait_loader<Trait: ?Sized + 'static>(self) -> AsTrait<Trait, Self, T> {
        AsTrait(self, PhantomData, PhantomData)
    }

}*/

/// type erased version of [Factory](chassis::Factory)
pub(crate) trait AnyFactory {
    fn load(&self, injector: &Injector) -> Box<dyn Any>;
}

pub(crate) struct AnyFactoryRef<T: ?Sized + 'static>(Arc<dyn Factory<T>>);

impl<T: ?Sized + 'static> AnyFactoryRef<T> {
    pub fn new(factory: impl Factory<T> + 'static) -> Self {
        Self(Arc::new(factory))
    }
}

impl<T: ?Sized + 'static> AnyFactory for AnyFactoryRef<T> {
    fn load(&self, service_locator: &Injector) -> Box<dyn Any> {
        Box::new(self.0.load(service_locator))
    }
}

pub struct ExistingFactory<T: ?Sized + 'static>(pub Arc<T>);

impl<T: ?Sized + 'static> Factory<T> for ExistingFactory<T> {
    fn load(&self, _injector: &Injector) -> Arc<T> {
        Arc::clone(&self.0)
    }
}

pub struct CreatingFactory<T: 'static>(pub Box<dyn Fn(&Injector) -> T>);

impl<T: 'static> CreatingFactory<T> {
    pub fn new(function: impl Fn(&Injector) -> T + 'static) -> Self {
        Self(Box::new(function))
    }
}

impl<T: 'static> Factory<T> for CreatingFactory<T> {
    fn load(&self, injector: &Injector) -> Arc<T> {
        Arc::new(self.0(injector))
    }
}

/*pub struct AsTrait<T: ?Sized + 'static, U: T + 'static, L: Loader<U>>(L, PhantomData<T>);

impl<T: ?Sized + 'static, U, L: Loader<U>> Loader<T> for AsTrait<T, U, L> {
    fn load(&self, service_locator: &Injector) -> Arc<T> {
        self.0.load(service_locator).into()
    }
}*/

pub struct AsTrait<T: ?Sized + 'static, U: 'static, L: Factory<U>>(
    L,
    PhantomData<U>,
    PhantomData<T>,
);

impl<T: ?Sized + 'static, U, L: Factory<U>> AsTrait<T, U, L> {
    pub fn new(factory: L) -> Self {
        Self(factory, PhantomData, PhantomData)
    }
}

impl<T: ?Sized + 'static, U, L: Factory<U>> Factory<U> for AsTrait<T, U, L>
where
    Arc<U>: From<Arc<T>>,
{
    fn load(&self, injector: &Injector) -> Arc<U> {
        self.0.load(injector)
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // struct Dummy();
    //
    // trait DummyTrait { }
    //
    // impl DummyTrait for Dummy { }

    #[test]
    fn as_trait() {
        // let loader = FactoryLoader(Box::new(|sf| Dummy()));
        // let as_trait: Arc<Dummy> =
        //     Box::new(loader.into_trait_loader::<dyn DummyTrait>()).load();
    }
}