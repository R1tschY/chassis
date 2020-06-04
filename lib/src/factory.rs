use std::marker::PhantomData;
use std::sync::Arc;

use crate::Injector;
use std::any::Any;

pub(crate) trait Factory<T: ?Sized + 'static> {
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

/// Interface to type erased form of factory product
///
/// Is a interface to Arc<?>.
trait ProductAny: Any + 'static {
    fn clone_product(&self) -> Product;
    fn as_any(self: Box<Self>) -> Box<dyn Any>;
}

impl<T: ?Sized + 'static> ProductAny for Arc<T> {
    fn clone_product(&self) -> Product {
        Product::new(self.clone())
    }

    fn as_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

/// Product of a factory
pub struct Product(Box<dyn ProductAny>); // aka. Box<Arc<?>>

impl Product {
    pub fn new<T: ?Sized + 'static>(value: Arc<T>) -> Self {
        Self(Box::new(value))
    }

    pub fn unwrap<T: ?Sized + 'static>(self) -> Arc<T> {
        *self.0.as_any().downcast::<Arc<T>>().unwrap()
    }
}

impl Clone for Product {
    fn clone(&self) -> Self {
        self.0.clone_product()
    }
}

/// type erased version of [Factory](chassis::Factory)
pub trait AnyFactory {
    fn load(&self, injector: &Injector) -> Product;
}

pub type AnyFactoryRef = Arc<dyn AnyFactory>;

pub(crate) fn to_any_factory<T: ?Sized + 'static, U: Factory<T> + 'static>(
    other: U,
) -> AnyFactoryRef {
    Arc::new(AnyFactoryImpl::<T, U>(other, PhantomData))
}

pub(crate) struct AnyFactoryImpl<T: ?Sized + 'static, U: Factory<T> + 'static>(U, PhantomData<T>);

impl<T: ?Sized + 'static, U: Factory<T> + 'static> AnyFactory for AnyFactoryImpl<T, U> {
    fn load(&self, service_locator: &Injector) -> Product {
        Product::new(self.0.load(service_locator))
    }
}

pub(crate) struct ConstantFactory<T: ?Sized + 'static>(pub Arc<T>);

impl<T: ?Sized + 'static> Factory<T> for ConstantFactory<T> {
    fn load(&self, _injector: &Injector) -> Arc<T> {
        Arc::clone(&self.0)
    }
}

pub(crate) struct CreatingFactory<T: 'static, F: Fn(&Injector) -> T>(pub F);

impl<T: 'static, F: Fn(&Injector) -> T> Factory<T> for CreatingFactory<T, F> {
    fn load(&self, injector: &Injector) -> Arc<T> {
        Arc::new(self.0(injector))
    }
}

pub(crate) struct ArcCreatingFactory<T, F>(pub F)
where
    T: ?Sized + 'static,
    F: Fn(&Injector) -> Arc<T>;

impl<T, F> Factory<T> for ArcCreatingFactory<T, F>
where
    T: ?Sized + 'static,
    F: Fn(&Injector) -> Arc<T>,
{
    fn load(&self, injector: &Injector) -> Arc<T> {
        self.0(injector)
    }
}

pub(crate) struct BoxCreatingFactory<T, F>(pub F)
where
    T: ?Sized + 'static,
    F: Fn(&Injector) -> Box<T>;

impl<T, F> Factory<T> for BoxCreatingFactory<T, F>
where
    T: ?Sized + 'static,
    F: Fn(&Injector) -> Box<T>,
{
    fn load(&self, injector: &Injector) -> Arc<T> {
        self.0(injector).into()
    }
}

/*pub struct AsTrait<T: ?Sized + 'static, U: T + 'static, L: Loader<U>>(L, PhantomData<T>);

impl<T: ?Sized + 'static, U, L: Loader<U>> Loader<T> for AsTrait<T, U, L> {
    fn load(&self, service_locator: &Injector) -> Arc<T> {
        self.0.load(service_locator).into()
    }
}*/

pub(crate) struct AsTrait<T: ?Sized + 'static, U: 'static, L: Factory<U>>(
    L,
    PhantomData<U>,
    PhantomData<T>,
);

// impl<T: ?Sized + 'static, U, L: Factory<U>> AsTrait<T, U, L> {
//     pub fn new(factory: L) -> Self {
//         Self(factory, PhantomData, PhantomData)
//     }
// }

impl<T: ?Sized + 'static, U, L: Factory<U>> Factory<U> for AsTrait<T, U, L>
where
    Arc<U>: From<Arc<T>>,
{
    fn load(&self, injector: &Injector) -> Arc<U> {
        self.0.load(injector)
    }
}
