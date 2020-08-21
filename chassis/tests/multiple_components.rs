use chassis::integration;

#[derive(PartialEq, Debug)]
pub struct StringArg(String);

#[derive(PartialEq, Debug)]
pub struct IntArg(i32);

#[derive(PartialEq, Debug)]
pub struct Dummy {
    string: StringArg,
    int: IntArg,
}

#[integration]
mod int_mod {
    use super::*;

    pub struct DummyProvider1;
    pub struct DummyProvider2;

    impl DummyProvider1 {
        pub fn provide_int() -> IntArg {
            IntArg(42)
        }
    }

    impl DummyProvider2 {
        pub fn provide_string() -> StringArg {
            StringArg("Test".to_string())
        }

        pub fn provide_dummy(string: StringArg, int: IntArg) -> Dummy {
            Dummy { string, int }
        }
    }

    pub trait DummyFactory1 {
        fn resolve_dummy(&self) -> Dummy;
    }

    pub trait DummyFactory2 {
        fn resolve_dummy(&self) -> Dummy;
    }
}

#[test]
fn check() {
    use crate::int_mod::DummyFactory1;
    use crate::int_mod::DummyFactory2;

    let injector1 = crate::int_mod::DummyFactory1Impl::new();
    let injector2 = crate::int_mod::DummyFactory2Impl::new();
    let dummy1 = injector1.resolve_dummy();
    let dummy2 = injector2.resolve_dummy();
    assert_eq!(dummy1, dummy2);
}
