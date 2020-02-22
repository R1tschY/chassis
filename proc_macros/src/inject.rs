use proc_macro::{TokenStream};

use syn::{Ident, Type};

use crate::syn_ext::IdentExt;

const INJECT_META_PREFIX: &'static str = "__injectmeta_";
const INJECT_PREFIX: &'static str = "__inject_";

struct InjectFnArg {
    name: Option<Ident>,
    ty: Type,
}

struct InjectFn {
    function: syn::ItemFn,
    name: Ident,
    inputs: Vec<InjectFnArg>,
    output: Type,
}

pub fn factory(input: TokenStream) -> TokenStream {
    let mut path: syn::Path = syn::parse(input).unwrap();

    let mut last_seg = path.segments.last_mut().expect("expected a path with at least one segment");
    last_seg.ident = last_seg.ident.prepend(INJECT_PREFIX);

    (quote! { #path }).into()
}

pub fn inject(input: TokenStream) -> TokenStream {
    let function: syn::ItemFn = syn::parse(input).unwrap();
    codegen_classfn(parse_sig(function))
}

fn parse_sig(function: syn::ItemFn) -> InjectFn {
    let inputs: Vec<_> = function.sig.inputs.iter().map(|input| {
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

    let rty: Type = match &function.sig.output {
        syn::ReturnType::Default => panic!("return type required"),
        syn::ReturnType::Type(_, ty) => (ty as &Type).clone(),
    };

    InjectFn {
        name: function.sig.ident.clone(),
        inputs,
        output: rty,
        function,
    }
}

fn codegen_classfn(sig: InjectFn) -> TokenStream {
    let userfn = &sig.function;
    let _rty = &sig.output;

    let userfn_name = &sig.name;
    let metafn_name = userfn_name.prepend(INJECT_META_PREFIX);
    let injectfn_name = userfn_name.prepend(INJECT_PREFIX);

    let resolves = sig.inputs.iter().map(|input| {
        let ty = &input.ty;
        // TODO: check if T is in scope
        quote! { __sl__.resolve_to::<#ty>() }
    });

    let code = quote! {
        #userfn

        pub fn #metafn_name() -> (String, std::any::TypeId) {
            ("<no name>".to_string(), std::any::TypeId::of::<Self>())
        }

        pub fn #injectfn_name(__sl__: &chassis::ServiceLocator) -> Self {
            Self::#userfn_name(#(#resolves),*)
        }
    };
    code.into()
}