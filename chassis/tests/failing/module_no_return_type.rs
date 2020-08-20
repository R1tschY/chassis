use chassis::integration;

pub struct Dummy;

#[integration]
mod int_mod {
    use super::*;

    pub struct DummyProvider;

    impl DummyProvider {
        pub fn provide_string() {
            ()
        }

        pub fn provide_dummy() -> Dummy {
            Dummy
        }
    }

    pub trait DummyFactory {
        fn resolve_dummy(&self) -> Dummy;
    }
}

fn main() {}
