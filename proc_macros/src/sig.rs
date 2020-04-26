use syn::{GenericArgument, Ident, PathArguments, PathSegment, Type};

pub struct InjectFnArg {
    pub name: Option<Ident>,
    pub ty: Type,
}

pub struct InjectFn {
    pub name: Ident,
    pub inputs: Vec<InjectFnArg>,
    pub output: InjectType,
}

pub struct InjectType {
    pub ty: Type,
    pub wrapper: Option<WrapperType>,
}

pub enum WrapperType {
    Arc,
    Box,
}

pub fn parse_sig(sig: &syn::Signature) -> InjectFn {
    let inputs: Vec<_> = sig
        .inputs
        .iter()
        .map(|input| {
            let (ident, ty) = match input {
                syn::FnArg::Typed(arg) => match *arg.pat {
                    syn::Pat::Ident(ref pat) => (Some(pat.ident.clone()), &arg.ty),
                    syn::Pat::Wild(_) => (None, &arg.ty),
                    _ => panic!("invalid use of pattern"),
                },
                _ => unreachable!("only usable on functions"),
            };

            InjectFnArg {
                name: ident,
                ty: *ty.clone(),
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
            ty: extract_single_generic_arg(seg),
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
            ty: ty.clone(),
            wrapper: None,
        }
    }
}
