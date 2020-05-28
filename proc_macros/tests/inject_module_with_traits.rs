#[macro_use]
extern crate assert_matches;
#[macro_use]
extern crate chassis;

use std::fmt::Debug;
use std::sync::Arc;

use chassis::Injector;

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
fn one_trait() {
    struct Module;

    #[module]
    impl Module {
        pub fn one_trait() -> Arc<dyn Trait1> {
            Arc::new(Class1)
        }
    }

    let injector = Injector::from_module(Module);
    assert_matches!(injector.resolve_type::<dyn Trait1>(), Some(_));
}

#[test]
fn two_traits_explicit() {
    struct Module;

    #[module]
    impl Module {
        pub fn class() -> Arc<Class1> {
            Arc::new(Class1)
        }

        pub fn trait1(cls: Arc<Class1>) -> Arc<dyn Trait1> {
            cls
        }

        pub fn trait2(cls: Arc<Class1>) -> Arc<dyn Trait2> {
            cls
        }
    }

    let injector = Injector::from_module(Module);
    assert_matches!(injector.resolve_type::<dyn Trait1>(), Some(_));
    assert_matches!(injector.resolve_type::<dyn Trait2>(), Some(_));
}

#[test]
fn two_traits() {
    struct Module;

    #[module]
    impl Module {
        pub fn class() -> Arc<Class1> {
            Arc::new(Class1)
        }

        pub fn trait1(cls: Arc<Class1>) -> Arc<dyn Trait1> {
            cls
        }

        pub fn trait2(cls: Arc<Class1>) -> Arc<dyn Trait2> {
            cls
        }
    }

    let injector = Injector::from_module(Module);
    assert_matches!(injector.resolve_type::<dyn Trait1>(), Some(_));
    assert_matches!(injector.resolve_type::<dyn Trait2>(), Some(_));
}
