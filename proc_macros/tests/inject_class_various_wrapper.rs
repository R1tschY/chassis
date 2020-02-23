#[macro_use] extern crate chassis;
#[macro_use] extern crate assert_matches;

use chassis::{ServiceLocator};
use chassis::FactoryLoader;
use std::sync::Arc;

#[derive(Debug, Clone)]
struct Class1();

impl Class1 {
    #[inject]
    pub fn new() -> Self { Self() }
}

#[derive(Debug)]
struct Class2();

impl Class2 {
    #[inject]
    pub fn new(_: Arc<Class1>, _: Option<Arc<Class1>>/*, _: ProviderPtr<Class1>*/) -> Self {
        Self()
    }
}

#[test]
fn inject_function_resolve() {
    let mut sl = ServiceLocator::new();
    sl.register(FactoryLoader(Box::new(Class1::__inject_new)));
    sl.register(FactoryLoader(Box::new(Class2::__inject_new)));

    assert!(sl.contains::<Class2>());
    assert_matches!(sl.resolve::<Class2>(), Some(_))
}
