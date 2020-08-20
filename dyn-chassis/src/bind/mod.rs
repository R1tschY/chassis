pub mod annotation;
pub mod binder;
pub mod binding;
pub mod linker;

#[cfg(test)]
mod tests {
    use crate::{AnonymousModule, Injector};

    struct Class;

    trait Trait1 {}
    impl Trait1 for Class {}

    #[cfg(nightly_unsize)]
    #[test]
    fn linked_binding_should_resolve_trait() {
        let injector = Injector::builder()
            .module(AnonymousModule::new(|binder| {
                binder.bind::<Class>().to_instance(Class);
                binder.bind::<dyn Trait1>().to_type::<Class>();
            }))
            .build()
            .unwrap();

        assert!(injector.resolve_type::<dyn Trait1>().is_some());
    }
}
