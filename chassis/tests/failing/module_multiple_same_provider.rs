use chassis::integration;

pub struct Dummy;

#[integration]
mod mut_self {
    use super::*;

    pub struct DummyProvider;

    impl DummyProvider {
        pub fn provide_dummy1() -> Dummy {
            Dummy
        }
        pub fn provide_dummy2() -> Dummy {
            Dummy
        }
    }

    pub trait DummyFactory {
        fn resolve_dummy(&self) -> Dummy {
            Dummy
        }
    }
}

fn main() {}
