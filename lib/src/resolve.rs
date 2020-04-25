use std::sync::Arc;

use crate::Injector;

pub trait ResolveFrom {
    fn resolve_from(_: &Injector) -> Self;
}

// impl<T: ?Sized + Clone + 'static> ResolveFrom for ProviderPtr<T> {
//     fn resolve_from(scope: &Injector) -> Self {
//         scope.to_provider()
//     }
// }

impl<T: ?Sized + 'static> ResolveFrom for Arc<T> {
    fn resolve_from(scope: &Injector) -> Self {
        scope.resolve().unwrap()
    }
}

impl<T: ?Sized + 'static> ResolveFrom for Option<Arc<T>> {
    fn resolve_from(scope: &Injector) -> Self {
        scope.resolve()
    }
}
