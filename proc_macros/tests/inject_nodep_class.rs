// #![feature(trace_macros)]
// trace_macros!(true);

#[macro_use]
extern crate assert_matches;
#[macro_use]
extern crate chassis;

use chassis::{AnonymousModule, Injector};

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
        Dummy::__injectbind_new(binder);
    }));

    assert_matches!(injector.resolve_type::<Dummy>(), Some(_));
}
