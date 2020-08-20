use std::sync::Arc;

use dyn_chassis::{module, Binder, Named};
use dyn_chassis::{BindAnnotation, Injector};

#[derive(Debug, PartialEq)]
struct Class1(String, String);

#[derive(Debug)]
struct Transactional;

impl BindAnnotation for Transactional {}

struct Module;

#[rustfmt::skip]
#[module]
impl Module {
    #[annotation(Named("parameter1"))]
    pub fn a1() -> String {
        "one".into()
    }

    pub fn class1(
        #[annotation(Named("parameter1"))] a1: Arc<String>,
        #[annotation(Transactional)] a2: Arc<String>
    ) -> Class1 {
        Class1(a1.to_string(), a2.to_string())
    }

    fn configure(binder: &mut Binder) {
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
