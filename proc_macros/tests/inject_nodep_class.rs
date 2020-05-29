use assert_matches::assert_matches;
use chassis::inject;
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
    }))
    .unwrap();

    assert_matches!(injector.resolve_type::<Dummy>(), Some(_));
}
