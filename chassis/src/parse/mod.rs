use proc_macro::TokenStream;

use syn::export::TokenStream2;
use syn::{ImplItem, Item, ItemImpl, ItemTrait, ReturnType, TraitItem, Type};

use crate::codegen::codegen_component_impl;
use crate::container::IocContainer;
use crate::errors::{ChassisError, ChassisResult};
use crate::model::{
    Binding, Block, ComponentTrait, Dependency, Implementation, InjectionPoint, Module, Request,
    StaticKey,
};
use crate::parse::signature::process_sig;

mod arguments;
mod attributes;
mod signature;

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
                    panic!(
                        "Default implementation of {} in component {} not allowed",
                        method.sig.ident, trait_block.ident
                    )
                }

                // TODO: Check for &self
                requests.push(parse_signature(&method.sig).unwrap()) // TODO: no unwrap()
            }
            TraitItem::Type(_) => panic!(
                "Associated type not allowed in component {}",
                trait_block.ident
            ),
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
