use std::sync::Arc;

use assert_matches::assert_matches;
use chassis::Injector;
use chassis::{inject, AnonymousModule};

#[derive(Debug)]
struct Class;

impl Class {
    #[inject]
    pub fn new(arc: Arc<u8> /*option: Option<Arc<u8>> , _: ProviderPtr<u8>*/) -> Self {
        assert_eq!(42, *arc);
        // assert_eq!(Some(42), option);
        Self
    }
}

#[test]
fn inject_function_resolve() {
    let injector = Injector::from_module(AnonymousModule::new(|binder| {
        binder.bind::<u8>().to_instance(42);
        Class::__injectbind_new(binder);
    }))
    .unwrap();

    assert_matches!(injector.resolve_type::<Class>(), Some(_))
}
