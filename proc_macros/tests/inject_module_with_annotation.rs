use std::sync::Arc;

use assert_matches::assert_matches;
use chassis::{module, Binder, Named};
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

    fn configure(binder: &mut Binder) {
        binder.bind::<String>().annotated_with(Named("parameter1")).to_instance("one".into());
        binder.bind::<String>().annotated_with(Transactional).to_instance("two".into());
    }
}

#[test]
fn from_arc() {
    let injector = Injector::from_module(Module).unwrap();

    assert_matches!(injector.resolve_type::<Class1>(), Some(_))
}
