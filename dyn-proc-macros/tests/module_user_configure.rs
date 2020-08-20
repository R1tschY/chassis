use std::sync::Arc;

use dyn_chassis::Injector;
use dyn_chassis::{module, Binder};

#[derive(Debug, PartialEq)]
struct Class1(u8);

// mixed

struct MixedModule;

#[rustfmt::skip]
#[module]
impl MixedModule {
    pub fn class1(
        answer: Arc<u8>,
    ) -> Class1 {
        Class1(*answer)
    }

    fn configure(binder: &mut Binder) {
        binder.bind::<u8>().to_instance(42);
    }
}

#[test]
fn mixed() {
    let injector = Injector::from_module(MixedModule).unwrap();
    assert_eq!(
        Some(Arc::new(Class1(42))),
        injector.resolve_type::<Class1>()
    )
}

// static

struct StaticModule;

#[rustfmt::skip]
#[module]
impl StaticModule {
    fn configure(binder: &mut Binder) {
        binder.bind::<u8>().to_instance(42);
    }
}

#[test]
fn static_() {
    let injector = Injector::from_module(StaticModule).unwrap();
    assert_eq!(Some(Arc::new(42)), injector.resolve_type::<u8>())
}

// non-static

struct NonStaticModule(u8);

#[rustfmt::skip]
#[module]
impl NonStaticModule {
    fn configure(&self, binder: &mut Binder) {
        binder.bind::<u8>().to_instance(self.0);
    }
}

#[test]
fn non_static() {
    let injector = Injector::from_module(NonStaticModule(24)).unwrap();
    assert_eq!(Some(Arc::new(24)), injector.resolve_type::<u8>())
}
