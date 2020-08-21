use core::fmt;
use core::hash::{Hash, Hasher};

use proc_macro2::Ident;

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

/// place where injection happens
///
/// For example a factory signature
pub struct InjectionPoint {
    /// a name for the injectee
    pub qualifier: String,

    /// Dependencies from function signature
    pub deps: Vec<Dependency>,
}

/// Dependency on key to be injected
///
/// Part of injection point
pub struct Dependency {
    /// Key for injection
    pub key: StaticKey,
    /// index of parameter in injection point
    pub parameter_index: u8,
}

/// Kind of binding used
///
/// Used for inspection and error messages. Analog to [Implementation].
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum BindingType {
    Factory,
    Instance,
    Linked,
}

/// Implementation for binding
pub enum Implementation {
    Factory {
        rty: syn::Type,
        module: Box<syn::Type>,
        func: Ident,
        injection_point: InjectionPoint,
    },
    //Instance,
    Linked(StaticKey),
}

/// Bind a implementation to a key
pub struct Binding {
    pub key: StaticKey,
    pub implementation: Implementation,
}

/// Group of bindings
pub struct Module {
    pub name: syn::TypePath,
    pub bindings: Vec<Binding>,
}

/// One injector specification entry.
pub struct Request {
    pub name: syn::Ident,
    pub key: StaticKey,
}

/// Closed collection of bindings and requests.
///
/// Components are implementing Injectors with bindings.
pub struct ComponentTrait {
    pub requests: Vec<Request>,
    pub trait_name: syn::Ident,
}

/// Definition block of components and modules
pub struct Block {
    pub modules: Vec<Module>,
    pub components: Vec<ComponentTrait>,
}
