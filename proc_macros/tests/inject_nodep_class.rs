// #![feature(trace_macros)]
// trace_macros!(true);

#[macro_use]
extern crate assert_matches;
#[macro_use]
extern crate chassis;

use std::any::TypeId;

use chassis::FactoryLoader;
use chassis::ServiceLocator;

#[derive(Debug)]
struct Dummy();

impl Dummy {
    #[inject]
    pub fn new() -> Self {
        Self()
    }
}

#[test]
fn inject_function_resolve() {
    let mut sl = ServiceLocator::new();
    sl.register(FactoryLoader(Box::new(Dummy::__inject_new)));

    assert_matches!(sl.resolve::<Dummy>(), Some(_));
}

#[test]
fn inject_function_meta() {
    let (_name, type_id) = Dummy::__injectmeta_new();

    assert_eq!(TypeId::of::<Dummy>(), type_id)
}
