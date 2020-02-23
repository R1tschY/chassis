use crate::service_locator::ServiceLocator;

pub trait Factory<T: 'static> {
    fn create(service_locator: &ServiceLocator) -> T;
}
