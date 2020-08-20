use chassis::integration;

pub struct StringArg(String);
pub struct IntArg(i32);

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

    pub trait DummyFactory {
        fn resolve_dummy(&self) -> Dummy;
    }
}

#[test]
fn check() {
    use crate::int_mod::DummyFactory;

    let injector = crate::int_mod::DummyFactoryImpl::new();
    let dummy = injector.resolve_dummy();
    assert_eq!("Test", &dummy.string.0);
    assert_eq!(42, dummy.int.0);
}
