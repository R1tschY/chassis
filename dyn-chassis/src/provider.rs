use std::sync::Arc;

/// JSR-330-like Provider interface
///
/// https://javax-inject.github.io/javax-inject/api/javax/inject/Provider.html
pub trait Provider<T: ?Sized + 'static> {
    fn get(&self) -> Option<Arc<T>>;
}

pub struct ProviderPtr<T: ?Sized + 'static>(Box<dyn Provider<T>>);

impl<T: ?Sized + 'static> ProviderPtr<T> {
    pub fn new(provider: impl Provider<T> + 'static) -> Self {
        Self(Box::new(provider))
    }
}

impl<T: ?Sized + 'static> std::ops::Deref for ProviderPtr<T> {
    type Target = dyn Provider<T> + 'static;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}
