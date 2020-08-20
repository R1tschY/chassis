use crate::container::IocContainer;
use crate::signature::process_sig;
use crate::syn_ext::IdentExt;
use crate::utils::to_tokens;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use std::fmt;
use std::hash::{Hash, Hasher};
use syn::export::TokenStream2;
use syn::{ImplItem, Item, ItemImpl, ItemTrait, ReturnType, TraitItem, Type};

#[derive(Debug)]
enum ChassisError {
    // TODO: allow multiple errors and track span
    MissingReturnTypeInComponent,
    TypeItemInComponent,
    DefaultImplementationInComponent,
}

type ChassisResult<T> = Result<T, ChassisError>;

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
        module: Box<Type>,
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

fn drain_where<T: Clone, F: Fn(&T) -> bool>(v: &mut Vec<T>, f: F) -> Vec<T> {
    // TODO: use Vec::drain_filter when stabilised
    let res: Vec<T> = v.iter().filter(|x| f(x)).cloned().collect();
    v.retain(|x| !f(x));
    res
}

pub fn eq_attr_name(attr: &syn::Attribute, seg0: &str) -> bool {
    let segs = &attr.path.segments;
    segs.len() == 1 && &segs[0].ident.to_string() == seg0
}

/// Main macro for compile time dependency injection.
///
/// Creates implementation for component trait.
pub fn integration(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut mod_block: syn::ItemMod = parse_macro_input!(input);

    let mut mod_impl = match &mut mod_block.content {
        Some((_, items)) => items,
        None => panic!("Expected module implementation when using integration attribute"),
    };

    // Parse components and modules
    let block = parse_block(&mut mod_impl);

    // analyse
    let modules = block.modules;
    let mut container = IocContainer::new();
    for module in modules {
        container.add_module(module);
    }

    // generate
    let component_impls: Vec<TokenStream2> = block
        .components
        .into_iter()
        .map(|comp| codegen_component_impl(comp, &container))
        .collect();

    // generate result
    let mod_name = &mod_block.ident;
    let mod_vis = &mod_block.vis;
    (quote! {
        #mod_vis mod #mod_name {
            #(#mod_impl)*

            #(#component_impls)*
        }
    })
    .into()
}

fn parse_block(mod_impl: &mut Vec<Item>) -> Block {
    let mut components: Vec<ComponentTrait> = vec![];
    let mut modules: Vec<Module> = vec![];
    for item in mod_impl {
        match item {
            // module definition
            Item::Impl(impl_block) => {
                // TODO: implicit?
                let attrs = drain_where(&mut impl_block.attrs, |attr| {
                    eq_attr_name(attr, "static_module")
                });
                if attrs.len() > 1 {
                    panic!("More than one static_module attribute found");
                } else {
                    modules.push(parse_module(attrs.into_iter().next(), impl_block));
                }
            }

            // component definition
            Item::Trait(trait_block) => {
                // TODO: implicit?
                let attrs = drain_where(&mut trait_block.attrs, |attr| {
                    eq_attr_name(attr, "static_component")
                });
                if attrs.len() > 1 {
                    panic!("More than one static_component attribute found");
                } else {
                    components.push(parse_component(attrs.into_iter().next(), trait_block));
                }
            }

            _ => (),
        }
    }

    Block {
        modules,
        components,
    }
}

fn codegen_component_impl(component: ComponentTrait, container: &IocContainer) -> TokenStream2 {
    let trait_name = component.trait_name;
    let impl_name = trait_name.append("Impl");
    let impl_items: Vec<_> = component
        .requests
        .into_iter()
        .map(|request| codegen_request_impl(request, container))
        .collect();

    let tokens = quote! {
        pub struct #impl_name;  // TODO: use visibility of trait

        impl #impl_name {
            pub fn new() -> Self { Self }
        }

        impl #trait_name for #impl_name {
            #(#impl_items)*
        }

        impl Default for #impl_name {
            fn default() -> Self {
                Self::new()
            }
        }
    };
    tokens
}

fn codegen_request_impl(request: Request, container: &IocContainer) -> TokenStream2 {
    let impl_code = codegen_key_impl(&request.key, container);
    let rty = &request.key.ty;
    let name = &request.name;
    let span = request.name.span(); // TODO: use Signature as span
    quote_spanned! {span=>
        fn #name(&self) -> #rty {
            #impl_code
        }
    }
}

fn codegen_key_impl(key: &StaticKey, container: &IocContainer) -> TokenStream2 {
    let binding = if let Some(binding) = container.resolve(key) {
        binding
    } else {
        panic!("Missing binding for `{}` to resolve `TODO`", key);
    };

    codegen_impl(binding, container)
}

fn codegen_impl(implementation: &Implementation, container: &IocContainer) -> TokenStream2 {
    match implementation {
        Implementation::Factory {
            rty: _,
            module,
            func,
            injection_point,
        } => {
            let dep_impls: Vec<TokenStream2> = injection_point
                .deps
                .iter()
                .map(|dep| codegen_key_impl(&dep.key, container))
                .collect();

            quote! {
                #module::#func(#(#dep_impls),*)
            }
        }
        Implementation::Linked(key) => codegen_key_impl(key, container),
    }
}

fn parse_signature(sig: &syn::Signature) -> ChassisResult<Request> {
    // TODO: len(args) == 0
    Ok(Request {
        key: match &sig.output {
            ReturnType::Default => return Err(ChassisError::MissingReturnTypeInComponent),
            ReturnType::Type(_, ty) => StaticKey::new(ty.clone()),
        },
        name: sig.ident.clone(),
    })
}

pub fn parse_component(_attr: Option<syn::Attribute>, trait_block: &ItemTrait) -> ComponentTrait {
    // TODO: parse attr for module names

    // TODO: check for generics / lifetimes / unsafe / auto / supertraits
    let mut requests: Vec<Request> = vec![];
    for item in trait_block.items.iter() {
        match item {
            TraitItem::Method(method) => {
                if method.default.is_some() {
                    panic!()
                    // TODO: return Err(ChassisError::DefaultImplementationInComponent);
                }

                // TODO: Check for &self
                requests.push(parse_signature(&method.sig).unwrap()) // TODO: no unwrap()
            }
            TraitItem::Type(_) => {
                panic!()
                // TODO: return Err(ChassisError::TypeItemInComponent);
            }
            _ => (),
        }
    }

    ComponentTrait {
        requests,
        trait_name: trait_block.ident.clone(),
    }
}

pub fn parse_module(_attr: Option<syn::Attribute>, impl_block: &mut ItemImpl) -> Module {
    // TODO: parse attr
    // TODO: check for generics / lifetimes / unsafe / auto / supertraits

    let module_id = impl_block.self_ty.clone();
    let bindings = impl_block
        .items
        .iter_mut()
        .map(|item| match item {
            ImplItem::Method(method) => {
                let inject_fn = process_sig(method);
                Binding {
                    key: StaticKey::new(Box::new(inject_fn.output.outer_ty.clone())), // TODO: inner type must be used
                    implementation: Implementation::Factory {
                        rty: inject_fn.output.outer_ty.clone(),
                        module: module_id.clone(),
                        func: inject_fn.name.clone(),
                        injection_point: InjectionPoint {
                            qualifier: inject_fn.name.to_string(),
                            deps: inject_fn
                                .inputs
                                .into_iter()
                                .enumerate()
                                .map(|(i, input)| {
                                    Dependency {
                                        parameter_index: i as u8,
                                        key: StaticKey::new(Box::new(input.ty.outer_ty)), // TODO: attr, inner type
                                    }
                                })
                                .collect(),
                        },
                    },
                }
            }
            _ => panic!("Unexpected item in chassis module definition"),
        })
        .collect();

    Module {
        name: match *impl_block.self_ty.clone() {
            Type::Path(path) if path.qself.is_none() => path,
            _ => panic!(
                "Expected simple type in static_module impl type, got {}",
                "TODO"
            ),
        },
        bindings,
    }
}
