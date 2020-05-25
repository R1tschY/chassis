// #![feature(trace_macros)]
// trace_macros!(true);

#[macro_use]
extern crate assert_matches;
#[macro_use]
extern crate chassis;

use chassis::{AnonymousModule, Injector, Key};

#[derive(Debug)]
struct Dummy();

impl Dummy {
    #[inject]
    pub fn new() -> Self {
        Self()
    }
}

/*#[test]
fn inject_function_resolve() {
    let injector = Injector::from_module(AnonymousModule::new(|binder| {
        binder.bind::<Dummy>().to_factory(Dummy::__inject_new);
    }));

    assert_matches!(injector.resolve::<Dummy>(), Some(_));
}

#[test]
fn inject_function_meta() {
    let (_name, _deps, result) = Dummy::__injectmeta_new();

    assert_eq!(Key::for_type::<Dummy>(), result)
}*/
