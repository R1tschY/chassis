use std::marker::PhantomData;
use std::sync::Arc;

pub struct FactoryResult1<T, Trait1>(Arc<T>, PhantomData<Trait1>)
where
    T: 'static,
    Trait1: ?Sized + 'static;

impl<T, Trait1> FactoryResult1<T, Trait1>
where
    T: 'static,
    Trait1: ?Sized + 'static,
{
    pub fn new(data: T) -> Self {
        Self(Arc::new(data), PhantomData)
    }
}

pub struct FactoryResult2<T, Trait1, Trait2>(Arc<T>, PhantomData<Trait1>, PhantomData<Trait2>)
where
    T: 'static,
    Trait1: ?Sized + 'static,
    Trait2: ?Sized + 'static;

impl<T, Trait1, Trait2> FactoryResult2<T, Trait1, Trait2>
where
    T: 'static,
    Trait1: ?Sized + 'static,
    Trait2: ?Sized + 'static,
{
    pub fn new(data: T) -> Self {
        Self(Arc::new(data), PhantomData, PhantomData)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Debug;

    #[derive(Debug)]
    struct Class1;

    trait Trait1: Debug {}
    trait Trait2: Debug {}
    trait Trait12: Trait1 + Trait2 {}
    trait TraitNotImplemented: Debug {}

    impl Trait1 for Class1 {}
    impl Trait2 for Class1 {}
    impl Trait12 for Class1 {}

    #[test]
    fn creation() {
        let a = FactoryResult2::<Class1, dyn Trait1, dyn Trait2>::new(Class1);
    }
}
