use assert_matches::assert_matches;

use std::sync::Arc;

use chassis::Module;
use chassis::{inject, Binder, Injector};

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
        Class1::__injectbind_new(binder);
        Class2::__injectbind_new(binder);
        Class3::__injectbind_new(binder);
    }
}

#[test]
fn inject_function_resolve() {
    let injector = Injector::builder().module(TestModule).build();

    assert!(injector.contains_type::<Class3>());
    assert_matches!(injector.resolve_type::<Class3>(), Some(_))
}
