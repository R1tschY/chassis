use std::sync::Arc;

use assert_matches::assert_matches;
use chassis::module;
use chassis::{Injector, Key};

#[derive(Debug)]
struct Class1();

impl Class1 {
    pub fn new() -> Self {
        Self()
    }
}

#[derive(Debug)]
struct Class2();

impl Class2 {
    pub fn new(_x: Arc<Class1>) -> Self {
        Self()
    }
}

#[derive(Debug)]
struct Class3();

impl Class3 {
    pub fn new(_x: Arc<Class2>) -> Self {
        Self()
    }
}

struct Module;

#[module]
impl Module {
    pub fn class1() -> Class1 {
        Class1::new()
    }
    pub fn class2(x: Arc<Class1>) -> Class2 {
        Class2::new(x)
    }
    pub fn class3(x: Arc<Class2>) -> Class3 {
        Class3::new(x)
    }
}

#[test]
fn inject_function_resolve() {
    let injector = Injector::from_module(Module).unwrap();

    assert!(injector.contains_type::<Class3>());
    assert_matches!(injector.resolve_type::<Class3>(), Some(_))
}

#[test]
fn test_debug() {
    let injector = Injector::from_module(Module).unwrap();
    let binding = injector.get_binding(Key::new::<Class3>()).unwrap();
    println!("member: {}", binding.injection_point().unwrap().member());
}
