use std::sync::Arc;

use assert_matches::assert_matches;
use chassis::module;
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
    let injector = Injector::from_module(Module).unwrap();

    assert_matches!(injector.resolve_type::<Class1>(), Some(_))
}

#[test]
fn from_box() {
    let injector = Injector::from_module(Module).unwrap();

    assert_matches!(injector.resolve_type::<Class2>(), Some(_))
}
