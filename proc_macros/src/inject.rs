use proc_macro::TokenStream;

use crate::sig::{parse_sig, InjectFn};
use crate::syn_ext::IdentExt;

pub const INJECT_META_PREFIX: &str = "__injectmeta_";
pub const INJECT_PREFIX: &str = "__inject_";

pub fn factory(input: TokenStream) -> TokenStream {
    let mut path: syn::Path = syn::parse(input).unwrap();

    let mut last_seg = path
        .segments
        .last_mut()
        .expect("expected a path with at least one segment");
    last_seg.ident = last_seg.ident.prepend(INJECT_PREFIX);

    (quote! { #path }).into()
}

pub fn inject(input: TokenStream) -> TokenStream {
    let function: syn::ImplItemMethod = syn::parse(input).unwrap();
    codegen_classfn(&function, parse_sig(&function.sig))
}

fn codegen_classfn(userfn: &syn::ImplItemMethod, sig: InjectFn) -> TokenStream {
    let injectfns = codegen_injectfns(&sig, true);

    let code = quote! {
        #userfn

        #injectfns
    };
    code.into()
}

pub fn codegen_injectfns(sig: &InjectFn, return_self: bool) -> proc_macro2::TokenStream {
    let userfn_name = &sig.name;
    let injectfn_name = userfn_name.prepend(INJECT_PREFIX);
    let metafn_name = userfn_name.prepend(INJECT_META_PREFIX);

    let resolves = sig.inputs.iter().map(|input| {
        let ty = &input.ty;
        // TODO: check if T is in scope
        quote! { __sl__.resolve_to::<#ty>() }
    });

    let code_metafn = quote! {
        pub fn #metafn_name() -> (String, std::any::TypeId) {
            ("<no name>".to_string(), std::any::TypeId::of::<Self>())
        }
    };

    let code_injectfn = if return_self {
        quote! {
            pub fn #injectfn_name(__sl__: &chassis::Injector) -> Self {
                Self::#userfn_name(#(#resolves),*)
            }
        }
    } else {
        let rty = &sig.output;
        quote! {
            pub fn #injectfn_name(__sl__: &chassis::Injector) -> #rty {
                Self::#userfn_name(#(#resolves),*)
            }
        }
    };

    quote! {
        #code_metafn

        #code_injectfn
    }
}
