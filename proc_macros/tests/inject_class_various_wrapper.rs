#[macro_use]
extern crate assert_matches;
#[macro_use]
extern crate chassis;

use std::sync::Arc;

use chassis::{Binder, Injector};
use chassis::{CreatingFactory, Module};

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
        binder.bind(CreatingFactory(Box::new(Class1::__inject_new)));
        binder.bind(CreatingFactory(Box::new(Class2::__inject_new)));
    }
}

#[test]
fn inject_function_resolve() {
    let injector = Injector::builder().module(TestModule).build();

    assert!(injector.contains::<Class2>());
    assert_matches!(injector.resolve::<Class2>(), Some(_))
}
