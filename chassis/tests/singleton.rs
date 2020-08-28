use chassis::integration;
use std::sync::atomic::{AtomicUsize, Ordering};

static INSTANCES: AtomicUsize = AtomicUsize::new(0);

pub struct InstanceCounter {
    instance: usize,
}

impl InstanceCounter {
    pub fn new() -> Self {
        Self {
            instance: INSTANCES.fetch_add(1, Ordering::SeqCst),
        }
    }

    pub fn instance(&self) -> usize {
        self.instance
    }

    pub fn instances() -> usize {
        INSTANCES.load(Ordering::SeqCst)
    }
}

#[integration]
mod int_mod {
    use super::*;
    use std::sync::Arc;

    pub struct TestProvider;

    impl TestProvider {
        #[singleton]
        pub fn provide_singleton() -> Arc<InstanceCounter> {
            Arc::new(InstanceCounter::new())
        }
    }

    pub trait TestFactory {
        fn provide_singleton(&self) -> Arc<InstanceCounter>;
    }
}

// TODO: Test multiple singletons
//   Test cyclic singletons

#[test]
fn check() {
    use crate::int_mod::TestFactory;

    let injector = crate::int_mod::TestFactoryImpl::new();
    injector.provide_singleton();
    injector.provide_singleton();
    let singleton = injector.provide_singleton();

    assert_eq!(0, singleton.instance());
    assert_eq!(1, InstanceCounter::instances());
}
