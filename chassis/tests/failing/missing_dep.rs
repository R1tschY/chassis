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

    pub struct DummyProvider;

    impl DummyProvider {
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

fn main() {}
