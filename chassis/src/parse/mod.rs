use syn::spanned::Spanned;
use syn::{ImplItem, Item, ItemImpl, ItemTrait, ReturnType, TraitItem, Type};

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

pub fn parse_block(mod_impl: &mut Vec<Item>) -> ChassisResult<Block> {
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
                    // TODO: hint for every attr
                    return Err(ChassisError::IllegalInput(
                        "More than one chassis attribute found".to_string(),
                        attrs[0].span(),
                    ));
                } else {
                    modules.push(parse_module(attrs.into_iter().next(), impl_block)?);
                }
            }

            // component definition
            Item::Trait(trait_block) => {
                // TODO: implicit?
                let attrs = drain_where(&mut trait_block.attrs, |attr| {
                    eq_attr_name(attr, "static_component")
                });
                if attrs.len() > 1 {
                    // TODO: hint for every attr
                    return Err(ChassisError::IllegalInput(
                        "More than one chassis attribute found".to_string(),
                        attrs[0].span(),
                    ));
                } else {
                    components.push(parse_component(attrs.into_iter().next(), trait_block)?);
                }
            }

            _ => (),
        }
    }

    Ok(Block {
        modules,
        components,
    })
}

fn parse_signature(sig: &syn::Signature) -> ChassisResult<Request> {
    // TODO: len(args) == 0
    Ok(Request {
        key: match &sig.output {
            ReturnType::Default => {
                return Err(ChassisError::IllegalInput(
                    "Return type is required".to_string(),
                    sig.span().clone(),
                ));
            }
            ReturnType::Type(_, ty) => StaticKey::new(ty.clone()),
        },
        name: sig.ident.clone(),
    })
}

pub fn parse_component(
    _attr: Option<syn::Attribute>,
    trait_block: &ItemTrait,
) -> ChassisResult<ComponentTrait> {
    // TODO: parse attr for module names

    // TODO: check for generics / lifetimes / unsafe / auto / supertraits
    let mut requests: Vec<Request> = vec![];
    for item in trait_block.items.iter() {
        match item {
            TraitItem::Method(method) => {
                if let Some(default) = &method.default {
                    return Err(ChassisError::IllegalInput(
                        "Default implementation not allowed".to_string(),
                        default.span().clone(),
                    ));
                }

                // TODO: Check for &self
                requests.push(parse_signature(&method.sig)?)
            }
            TraitItem::Type(type_item) => {
                return Err(ChassisError::IllegalInput(
                    "Associated type not allowed in component".to_string(),
                    type_item.span().clone(),
                ))
            }
            _ => (),
        }
    }

    Ok(ComponentTrait {
        requests,
        trait_name: trait_block.ident.clone(),
    })
}

pub fn parse_module(
    _attr: Option<syn::Attribute>,
    impl_block: &mut ItemImpl,
) -> ChassisResult<Module> {
    // TODO: parse attr
    // TODO: check for generics / lifetimes / unsafe / auto / supertraits

    let module_id = impl_block.self_ty.clone();
    let bindings: ChassisResult<Vec<_>> = impl_block
        .items
        .iter_mut()
        .map(|item| match item {
            ImplItem::Method(method) => {
                let inject_fn = process_sig(method);
                Ok(Binding {
                    key: StaticKey::new(Box::new(inject_fn.output.outer_ty.clone())), // TODO: inner type must be used
                    implementation: Implementation {
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
                })
            }
            _ => {
                return Err(ChassisError::IllegalInput(
                    "Unexpected item in chassis module definition".to_string(),
                    item.span().clone(),
                ))
            }
        })
        .collect();

    Ok(Module {
        name: match *impl_block.self_ty.clone() {
            Type::Path(path) if path.qself.is_none() => path,
            self_ty => {
                return Err(ChassisError::IllegalInput(
                    "Expected simple type in static_module impl type".to_string(),
                    self_ty.span().clone(),
                ))
            }
        },
        bindings: bindings?,
    })
}
