use chassis::{ServiceLocator, FactoryLoader};

trait ProgressBar {

}

trait Copier {

}

trait Converter {
    fn convert(&self, bytes: &[u8]) -> Vec<u8>;
}

struct Dummy();

impl Converter for Dummy {
    fn convert(&self, bytes: &[u8]) -> Vec<u8> {
        bytes.into()
    }
}

fn main() {
    let mut sl = ServiceLocator::new();
    sl.register(FactoryLoader(Box::new(|sl| Dummy())));

    sl.resolve::<Dummy>().unwrap().convert(&[1, 2, 3]);
}