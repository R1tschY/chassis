use chassis::module;
use chassis::Injector;

trait ProgressBar {}

trait Copier {}

trait Converter {
    fn convert(&self, bytes: &[u8]) -> Vec<u8>;
}

struct Dummy();

impl Dummy {
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

#[module]
impl DDModule {
    pub fn dummy() -> Dummy {
        Dummy::new()
    }
}

fn main() {
    let injector = Injector::builder().module(DDModule).build().unwrap();

    injector
        .resolve_type::<Dummy>()
        .unwrap()
        .convert(&[1, 2, 3]);
}
