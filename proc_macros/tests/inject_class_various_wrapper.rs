use std::sync::Arc;

use assert_matches::assert_matches;
use chassis::inject;
use chassis::Module;
use chassis::{Binder, Injector};

#[derive(Debug, Clone)]
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
    pub fn new(_: Arc<Class1>, _: Option<Arc<Class1>> /*, _: ProviderPtr<Class1>*/) -> Self {
        Self()
    }
}

struct TestModule;

impl Module for TestModule {
    fn configure(&self, binder: &mut Binder) {
        Class1::__injectbind_new(binder);
        Class2::__injectbind_new(binder);
    }
}

#[test]
fn inject_function_resolve() {
    let injector = Injector::from_module(TestModule).unwrap();

    assert!(injector.contains_type::<Class2>());
    assert_matches!(injector.resolve_type::<Class2>(), Some(_))
}
