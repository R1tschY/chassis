use lazy_static::lazy_static;
use std::any::{type_name, TypeId};
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref TYPE_ID_TO_TYPE_NAME: Mutex<HashMap<TypeId, &'static str>> =
        Mutex::new(HashMap::new());
}

pub(crate) fn save_type_name<T: ?Sized + 'static>() {
    TYPE_ID_TO_TYPE_NAME
        .lock()
        .unwrap()
        .insert(TypeId::of::<T>(), type_name::<T>());
}

pub(crate) fn get_type_name(id: TypeId) -> Option<&'static str> {
    TYPE_ID_TO_TYPE_NAME.lock().unwrap().get(&id).copied()
}
