use crate::{ServiceLocator, Provider, ProviderPtr};
use std::sync::Arc;


pub trait ResolveFrom {
    fn resolve_from(_: &ServiceLocator) -> Self;
}

// impl<T: ?Sized + Clone + 'static> ResolveFrom for ProviderPtr<T> {
//     fn resolve_from(scope: &ServiceLocator) -> Self {
//         scope.to_provider()
//     }
// }

impl<T: ?Sized + 'static> ResolveFrom for Arc<T> {
    fn resolve_from(scope: &ServiceLocator) -> Self {
        scope.resolve().unwrap()
    }
}

impl<T: ?Sized + 'static> ResolveFrom for Option<Arc<T>> {
    fn resolve_from(scope: &ServiceLocator) -> Self {
        scope.resolve()
    }
}