use std::any::{Any, TypeId};
use std::sync::Arc;

pub trait Scope {
    fn get(&self, tp: TypeId) -> Option<Box<dyn Any>>;

    fn resolve<T: ?Sized + 'static>(&self) -> Option<Arc<T>> {
        self.get(TypeId::of::<T>())
            .map(|any| *any.downcast::<Arc<T>>().unwrap())
    }

    // fn resolve_to<T: ?Sized + 'static, R: ResolveTo<T>>(&self) -> R {
    //     self.get(TypeId::of::<T>())
    //         .map(|any| R::resolve(*any.downcast::<Arc<T>>().unwrap()))
    // }
}

pub struct Singleton;
pub type Process = Singleton;
