use std::any::TypeId;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct Key(TypeId);

impl Key {
    pub fn for_type<T: ?Sized + 'static>() -> Self {
        Self(TypeId::of::<T>())
    }
}
