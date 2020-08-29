use chassis::integration;
use std::sync::atomic::{AtomicUsize, Ordering};

macro_rules! instance_counter_cls {
    ($name:ident, $instances_name:ident) => {
        static $instances_name: AtomicUsize = AtomicUsize::new(0);

        pub struct $name {
            instance: usize,
        }

        impl $name {
            pub fn new() -> Self {
                Self {
                    instance: $instances_name.fetch_add(1, Ordering::SeqCst),
                }
            }

            pub fn instance(&self) -> usize {
                self.instance
            }

            pub fn instances() -> usize {
                $instances_name.load(Ordering::SeqCst)
            }

            pub fn reset() {
                $instances_name.store(0, Ordering::SeqCst)
            }
        }
    };
}

instance_counter_cls!(InstanceCounter1, INSTANCES_1);
instance_counter_cls!(InstanceCounter2, INSTANCES_2);

#[integration]
mod int_mod {
    use super::*;
    use std::sync::Arc;

    pub struct TestProvider;

    impl TestProvider {
        #[singleton]
        pub fn provide_singleton2() -> Arc<InstanceCounter2> {
            Arc::new(InstanceCounter2::new())
        }

        #[singleton]
        pub fn provide_singleton1(_dep: Arc<InstanceCounter2>) -> Arc<InstanceCounter1> {
            Arc::new(InstanceCounter1::new())
        }
    }

    pub trait TestFactory {
        fn provide_singleton1(&self) -> Arc<InstanceCounter1>;
        fn provide_singleton2(&self) -> Arc<InstanceCounter2>;
    }
}

#[test]
fn check_dep() {
    use crate::int_mod::TestFactory;

    InstanceCounter1::reset();
    InstanceCounter2::reset();

    let injector = crate::int_mod::TestFactoryImpl::new();
    injector.provide_singleton1();
    injector.provide_singleton1();
    let singleton = injector.provide_singleton1();

    assert_eq!(0, singleton.instance());
    assert_eq!(1, InstanceCounter1::instances());
    assert_eq!(1, InstanceCounter2::instances());
}

#[test]
fn check_multiple() {
    use crate::int_mod::TestFactory;

    InstanceCounter1::reset();
    InstanceCounter2::reset();

    let injector = crate::int_mod::TestFactoryImpl::new();
    injector.provide_singleton1();
    injector.provide_singleton2();
    let singleton = injector.provide_singleton2();

    assert_eq!(0, singleton.instance());
    assert_eq!(1, InstanceCounter2::instances());
}
