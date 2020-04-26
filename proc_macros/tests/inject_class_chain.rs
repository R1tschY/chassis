#![feature(proc_macro_hygiene)]

#[macro_use]
extern crate assert_matches;
#[macro_use]
extern crate chassis;

use std::sync::Arc;

use chassis::Module;
use chassis::{Binder, Injector};

#[derive(Debug)]
struct Class1();

impl Class1 {
    #[inject]
    pub fn new() -> Self {
        Self()
    }
}

#[derive(Debug)]
struct Class2();

impl Class2 {
    #[inject]
    pub fn new(_x: Arc<Class1>) -> Self {
        Self()
    }
}

#[derive(Debug)]
struct Class3();

impl Class3 {
    #[inject]
    pub fn new(_x: Arc<Class2>) -> Self {
        Self()
    }
}

struct TestModule;

impl Module for TestModule {
    fn configure(&self, binder: &mut Binder) {
        binder.bind_factory(factory!(Class1::new));
        binder.bind_factory(factory!(Class2::new));
        binder.bind_factory(factory!(Class3::new));
    }
}

#[test]
fn inject_function_resolve() {
    let injector = Injector::builder().module(TestModule).build();

    assert!(injector.contains::<Class3>());
    assert_matches!(injector.resolve::<Class3>(), Some(_))
}
