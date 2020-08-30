use core::fmt;
use core::hash::{Hash, Hasher};

use crate::utils::to_tokens;

/// Key to something to inject
///
/// Used to reference dependencies
#[derive(Clone)]
pub struct StaticKey {
    /// Type to inject
    ty: Box<syn::Type>,
    ty_str: String,
}

impl StaticKey {
    pub fn new(ty: Box<syn::Type>) -> Self {
        Self {
            ty_str: to_tokens(&ty).to_string(),
            ty,
        }
    }

    pub fn type_(&self) -> &syn::Type {
        &self.ty
    }

    #[allow(dead_code)]
    pub fn type_string(&self) -> &str {
        &self.ty_str
    }
}

impl Hash for StaticKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ty_str.hash(state);
    }
}

impl PartialEq for StaticKey {
    fn eq(&self, other: &Self) -> bool {
        self.ty_str == other.ty_str
    }
}

impl Eq for StaticKey {}

impl fmt::Debug for StaticKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("StaticKey").field(&self.ty_str).finish()
    }
}

impl fmt::Display for StaticKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.ty_str)
    }
}
