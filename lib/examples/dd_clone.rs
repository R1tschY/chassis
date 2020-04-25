use chassis::{Binder, CreatingFactory, Injector, Module};

trait ProgressBar {}

trait Copier {}

trait Converter {
    fn convert(&self, bytes: &[u8]) -> Vec<u8>;
}

struct Dummy();

impl Dummy {
    //#[inject]
    pub fn new() -> Self {
        Self()
    }
}

impl Converter for Dummy {
    fn convert(&self, bytes: &[u8]) -> Vec<u8> {
        bytes.into()
    }
}

struct DDModule;

impl Module for DDModule {
    fn configure(&self, binder: &mut Binder) {
        binder.bind(CreatingFactory(Box::new(|_sl| Dummy::new())));
    }
}

fn main() {
    let injector = Injector::builder().module(DDModule).build();

    injector.resolve::<Dummy>().unwrap().convert(&[1, 2, 3]);
}
