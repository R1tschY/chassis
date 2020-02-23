use crate::ServiceLocator;

pub trait Module {
    fn configure(&self, sl: &mut ServiceLocator);
}
