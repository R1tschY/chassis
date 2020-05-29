#[macro_use]
extern crate assert_matches;
#[macro_use]
extern crate chassis;

use std::sync::Arc;

use chassis::{BindAnnotation, Injector};

#[derive(Debug)]
struct Class1(Arc<String>, Arc<String>);

#[derive(Debug)]
struct Transactional;

impl BindAnnotation for Transactional {}

struct Module;

#[rustfmt::skip]
#[module]
impl Module {
    pub fn class1(
        #[chassis(Named("parameter1"))] a1: Arc<String>,
        #[chassis(Transactional)] a2: Arc<String>
    ) -> Class1 {
        Class1(a1, a2)
    }
}

#[test]
fn from_arc() {
    let injector = Injector::from_module(Module);

    assert_matches!(injector.resolve_type::<Class1>(), Some(_))
}
