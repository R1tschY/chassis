use std::sync::Arc;

use chassis::{module, Binder, Named};
use chassis::{BindAnnotation, Injector};

#[derive(Debug, PartialEq)]
struct Class1(String, String);

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
        Class1(a1.to_string(), a2.to_string())
    }

    fn configure(binder: &mut Binder) {
        binder.bind::<String>().annotated_with(Named("parameter1")).to_instance("one".into());
        binder.bind::<String>().annotated_with(Transactional).to_instance("two".into());
    }
}

#[test]
fn from_arc() {
    let injector = Injector::from_module(Module).unwrap();

    assert_eq!(
        Some(Arc::new(Class1("one".into(), "two".into()))),
        injector.resolve_type::<Class1>(),
    )
}
