extern crate proc_macro;
#[macro_use] extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

use syn::{Ident, Type};

use crate::syn_ext::IdentExt;

mod syn_ext;

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

#[proc_macro_attribute]
pub fn inject(_args: TokenStream, input: TokenStream) -> TokenStream {
    let function: syn::ItemFn = syn::parse(input).unwrap();
    codegen_classfn(parse_sig(function))
}

fn parse_sig(function: syn::ItemFn) -> InjectFn {
    let mut inputs = Vec::new();
    for input in &function.sig.inputs {
        let (ident, ty) = match input {
            syn::FnArg::Typed(arg) => match *arg.pat {
                syn::Pat::Ident(ref pat) => (Some(pat.ident.clone()), &arg.ty),
                syn::Pat::Wild(_) => (None, &arg.ty),
                _ => panic!("invalid use of pattern")
            }
            _ => unreachable!("only usable on functions"),
        };

        inputs.push(InjectFnArg { name: ident, ty: *ty.clone() });
    }

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

/*    let code = quote! {
        impl Factory<#rty> for #name {
            fn create(sl: &chassis::ServiceLocator) -> Arc<#rty> {
                Arc::new(#rty(#inputs))
            }
        }
    }*/
}