#[macro_use]
extern crate assert_matches;
#[macro_use]
extern crate chassis;

use std::sync::Arc;

use chassis::Injector;

#[derive(Debug)]
struct Class1;

#[derive(Debug)]
struct Class2;

struct Module;

#[module]
impl Module {
    pub fn class1() -> Arc<Class1> {
        Arc::new(Class1)
    }

    pub fn class2() -> Box<Class2> {
        Box::new(Class2)
    }
}

#[test]
fn from_arc() {
    let injector = Injector::from_module(Module);

    assert_matches!(injector.resolve::<Class1>(), Some(_))
}

#[test]
fn from_box() {
    let injector = Injector::from_module(Module);

    assert_matches!(injector.resolve::<Class2>(), Some(_))
}
