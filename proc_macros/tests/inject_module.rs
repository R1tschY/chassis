#[macro_use] extern crate chassis;
#[macro_use] extern crate assert_matches;

use chassis::ServiceLocator;
use chassis::FactoryLoader;
use std::sync::Arc;

#[derive(Debug)]
struct Class1();

impl Class1 {
    pub fn new() -> Self { Self() }
}

#[derive(Debug)]
struct Class2();

impl Class2 {
    pub fn new(_x: Arc<Class1>) -> Self { Self() }
}

#[derive(Debug)]
struct Class3();

impl Class3 {
    pub fn new(_x: Arc<Class2>) -> Self {
        Self()
    }
}

struct Module();

#[module]
impl Module {
    pub fn class1() -> Class1 { Class1::new() }
    pub fn class2(x: Arc<Class1>) -> Class2 { Class2::new(x) }
    pub fn class3(x: Arc<Class2>) -> Class3 { Class3::new(x) }
}

#[test]
fn inject_function_resolve() {
    let mut sl = ServiceLocator::new();
    sl.install(&Module());

    assert!(sl.contains::<Class3>());
    assert_matches!(sl.resolve::<Class3>(), Some(_))
}
