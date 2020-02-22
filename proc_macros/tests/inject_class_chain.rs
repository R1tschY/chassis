// #![feature(trace_macros)]
// trace_macros!(true);
#![feature(proc_macro_hygiene)]

#[macro_use] extern crate chassis;
#[macro_use] extern crate assert_matches;

use chassis::ServiceLocator;
use chassis::FactoryLoader;
use std::sync::Arc;

#[derive(Debug)]
struct Class1();

impl Class1 {
    #[inject]
    pub fn new() -> Self { Self() }
}

#[derive(Debug)]
struct Class2();

impl Class2 {
    #[inject]
    pub fn new(_x: Arc<Class1>) -> Self { Self() }
}

#[derive(Debug)]
struct Class3();

impl Class3 {
    #[inject]
    pub fn new(_x: Arc<Class2>) -> Self {
        Self()
    }
}

#[test]
fn inject_function_resolve() {
    let mut sl = ServiceLocator::new();
    sl.register(FactoryLoader(Box::new(factory!(Class1::new))));
    sl.register(FactoryLoader(Box::new(factory!(Class2::new))));
    sl.register(FactoryLoader(Box::new(factory!(Class3::new))));

    assert!(sl.contains::<Class3>());
    assert_matches!(sl.resolve::<Class3>(), Some(_))
}
