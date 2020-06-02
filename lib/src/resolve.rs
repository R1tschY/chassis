use std::sync::Arc;

use crate::Key;

/// Resolve result into
pub trait ResolveInto<T: ?Sized + 'static> {
    fn resolve_into(result: Option<Arc<T>>, key: &Key) -> Self;
}

// impl<T: ?Sized + Clone + 'static> ResolveFrom for ProviderPtr<T> {
//     fn resolve_from(scope: &Injector) -> Self {
//         scope.to_provider()
//     }
// }

impl<T: ?Sized + 'static> ResolveInto<T> for Arc<T> {
    fn resolve_into(result: Option<Arc<T>>, key: &Key) -> Self {
        result.expect(&format!("Failed to resolve {:?}", key))
    }
}

impl<T: ?Sized + 'static> ResolveInto<T> for Option<Arc<T>> {
    fn resolve_into(result: Option<Arc<T>>, _key: &Key) -> Self {
        result
    }
}
