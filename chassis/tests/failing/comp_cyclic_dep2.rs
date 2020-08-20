use chassis::integration;

pub struct Dummy1;
pub struct Dummy2;

#[integration]
mod mut_self {
    use super::*;

    pub struct DummyProvider;

    impl DummyProvider {
        pub fn provide_dummy2(dummy: Dummy1) -> Dummy2 {
            Dummy2
        }

        pub fn provide_dummy1(dummy: Dummy2) -> Dummy1 {
            Dummy1
        }
    }

    pub trait DummyFactory {
        fn resolve_dummy(&self) -> Dummy1;
    }
}

fn main() {}
