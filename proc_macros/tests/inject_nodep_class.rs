// #![feature(trace_macros)]
// trace_macros!(true);

#[macro_use]
extern crate assert_matches;
#[macro_use]
extern crate chassis;

use chassis::meta::Binding;
use chassis::{AnonymousModule, Injector, Key};

#[derive(Debug)]
struct Dummy();

impl Dummy {
    #[inject]
    pub fn new() -> Self {
        Self()
    }
}

#[test]
fn inject_function_resolve() {
    let injector = Injector::from_module(AnonymousModule::new(|binder| {
        binder.use_binding(Dummy::__injectmeta_new());
    }));

    assert_matches!(injector.resolve_type::<Dummy>(), Some(_));
}

#[test]
fn inject_function_meta() {
    let binding: Binding = Dummy::__injectmeta_new();
    assert_eq!(Key::new::<Dummy>(), binding.key())
}
