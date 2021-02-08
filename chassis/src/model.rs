use proc_macro2::{Ident, Span};

use crate::key::StaticKey;

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

    // Span of type in injection point
    pub span: Span,

    /// index of parameter in injection point
    pub parameter_index: u8,
}

// /// Kind of binding used
// ///
// /// Used for inspection and error messages. Analog to [Implementation].
// #[derive(PartialEq, Copy, Clone, Debug)]
// pub enum BindingType {
//     Factory,
//     Instance,
//     Linked,
// }

/// Implementation for binding
pub struct Implementation {
    pub rty: syn::Type,
    pub module: Box<syn::Type>,
    pub func: Ident,
    pub injection_point: InjectionPoint,
    pub singleton: bool,
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
    /// function name
    pub name: syn::Ident,

    /// type of request
    pub ty: syn::Type,

    /// key to resolve (normalized type)
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
