use syn::{Ident, Type};

pub struct InjectFnArg {
    pub name: Option<Ident>,
    pub ty: Type,
}

pub struct InjectFn {
    pub name: Ident,
    pub inputs: Vec<InjectFnArg>,
    pub output: Type,
}

pub fn parse_sig(sig: &syn::Signature) -> InjectFn {
    let inputs: Vec<_> = sig.inputs.iter().map(|input| {
        let (ident, ty) = match input {
            syn::FnArg::Typed(arg) => match *arg.pat {
                syn::Pat::Ident(ref pat) => (Some(pat.ident.clone()), &arg.ty),
                syn::Pat::Wild(_) => (None, &arg.ty),
                _ => panic!("invalid use of pattern")
            }
            _ => unreachable!("only usable on functions"),
        };

        InjectFnArg { name: ident, ty: *ty.clone() }
    }).collect();

    let rty: Type = match &sig.output {
        syn::ReturnType::Default => panic!("return type required"),
        syn::ReturnType::Type(_, ty) => (ty as &Type).clone(),
    };

    InjectFn {
        name: sig.ident.clone(),
        inputs,
        output: rty,
    }
}