use syn::{Attribute, GenericArgument, Ident, PathArguments, PathSegment, Type};

pub struct InjectFnArg {
    pub name: Option<Ident>,
    pub attr: Option<Attribute>,
    pub ty: InjectType,
}

pub struct InjectFn {
    pub name: Ident,
    pub inputs: Vec<InjectFnArg>,
    pub output: InjectType,
}

pub struct InjectType {
    pub outer_ty: Type,
    pub inner_ty: Type,
    pub wrapper: Option<WrapperType>,
}

pub enum WrapperType {
    Arc,
    Box,
}

fn is_chassis_attr(attr: &Attribute) -> bool {
    let segs = &attr.path.segments;
    segs.len() == 1 && &segs[0].ident.to_string() == "chassis"
}

fn drain_where<T: Clone, F: Fn(&T) -> bool>(v: &mut Vec<T>, f: F) -> Vec<T> {
    // TODO: use Vec::drain_filter when stabilised
    let res: Vec<T> = v.iter().filter(|x| f(x)).cloned().collect();
    v.retain(|x| !f(x));
    res
}

/// Parse function signature and removes chassis annotations.
pub fn process_sig(sig: &mut syn::Signature) -> InjectFn {
    let inputs: Vec<_> = sig
        .inputs
        .iter_mut()
        .map(|input| {
            let (attrs, ident, ty) = match input {
                syn::FnArg::Typed(arg) => match *arg.pat {
                    syn::Pat::Ident(ref mut pat) => {
                        (&mut arg.attrs, Some(pat.ident.clone()), &arg.ty)
                    }
                    syn::Pat::Wild(_) => (&mut arg.attrs, None, &arg.ty),
                    _ => panic!("invalid use of pattern"),
                },
                _ => unreachable!("only usable on functions"),
            };

            let chassis_attrs: Vec<Attribute> = drain_where(attrs, is_chassis_attr);
            if chassis_attrs.len() > 1 {
                panic!("Only one chassis dependency annotation is allowed");
            }

            InjectFnArg {
                name: ident,
                attr: chassis_attrs.into_iter().next(),
                ty: parse_inject_type(ty),
            }
        })
        .collect();

    let rty: InjectType = match &sig.output {
        syn::ReturnType::Default => panic!("return type required"),
        // TODO: check for type: no lifetime, ...
        syn::ReturnType::Type(_, ty) => parse_inject_type(ty),
    };

    InjectFn {
        name: sig.ident.clone(),
        inputs,
        output: rty,
    }
}

// Parse generic with one type argument
fn extract_single_generic_arg(path_seg: &PathSegment) -> Type {
    if let PathArguments::AngleBracketed(args) = &path_seg.arguments {
        if args.args.len() == 1 {
            if let GenericArgument::Type(ty) = &args.args[0] {
                ty.clone()
            } else {
                panic!("Expected type argument for {}", path_seg.ident);
            }
        } else {
            panic!("Expected only one argument for {}", path_seg.ident);
        }
    } else {
        panic!("Expected angle argument for {}", path_seg.ident);
    }
}

fn find_wrapper_type(ty: &Type) -> Option<InjectType> {
    if let Type::Path(path) = ty {
        let segments = &path.path.segments;
        if segments.len() != 1 {
            return None;
        }

        let seg = &segments[0];
        let ptr_ty = if seg.ident == "Arc" {
            WrapperType::Arc
        } else if seg.ident == "Box" {
            WrapperType::Box
        } else {
            return None;
        };

        Some(InjectType {
            outer_ty: ty.clone(),
            inner_ty: extract_single_generic_arg(seg),
            wrapper: Some(ptr_ty),
        })
    } else {
        None
    }
}

fn parse_inject_type(ty: &Type) -> InjectType {
    if let Some(inject_ptr) = find_wrapper_type(ty) {
        inject_ptr
    } else {
        InjectType {
            outer_ty: ty.clone(),
            inner_ty: ty.clone(),
            wrapper: None,
        }
    }
}
