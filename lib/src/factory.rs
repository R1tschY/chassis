use crate::injector::Injector;

pub trait Factory<T: 'static> {
    fn create(service_locator: &Injector) -> T;
}
