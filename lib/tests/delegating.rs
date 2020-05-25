use chassis::{AnonymousModule, Injector};
use std::sync::Arc;

#[derive(Debug, Eq, PartialEq)]
struct Class1;

trait Trait1 {}
trait Trait2 {}
trait Trait12: Trait1 + Trait2 {}

impl Trait1 for Class1 {}
impl Trait2 for Class1 {}
impl Trait12 for Class1 {}

/*
#[test]
pub fn bind_to_trait() {
    let injector = Injector::from_module(AnonymousModule::new(|binder| {
        binder
            .bind::<dyn Trait1>()
            .to_arc_factory(|_| Arc::new(Class1));
    }));

    assert!(injector.resolve::<dyn Trait1>().is_some());
}
*/
