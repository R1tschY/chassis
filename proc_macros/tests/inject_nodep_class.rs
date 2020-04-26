// #![feature(trace_macros)]
// trace_macros!(true);

#[macro_use]
extern crate assert_matches;
#[macro_use]
extern crate chassis;

use chassis::{Binder, Injector, Key};
use chassis::{CreatingFactory, Module};

#[derive(Debug)]
struct Dummy();

impl Dummy {
    #[inject]
    pub fn new() -> Self {
        Self()
    }
}

struct TestModule;

impl Module for TestModule {
    fn configure(&self, binder: &mut Binder) {
        binder.bind(CreatingFactory(Box::new(Dummy::__inject_new)));
    }
}

#[test]
fn inject_function_resolve() {
    let injector = Injector::builder().module(TestModule).build();

    assert_matches!(injector.resolve::<Dummy>(), Some(_));
}

#[test]
fn inject_function_meta() {
    let (_name, _deps, result) = Dummy::__injectmeta_new();

    assert_eq!(Key::for_type::<Dummy>(), result)
}
