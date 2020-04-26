use proc_macro::TokenStream;

use crate::sig::{parse_sig, InjectFn, WrapperType};
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
    let userfn_name_str = sig.name.to_string();
    let injectfn_name = userfn_name.prepend(INJECT_PREFIX);
    let metafn_name = userfn_name.prepend(INJECT_META_PREFIX);

    let resolves = sig.inputs.iter().map(|input| {
        let ty = &input.ty;
        quote! { __sl__.resolve_to::<#ty>() }
    });
    let dep_keys = sig.inputs.iter().map(|input| {
        let ty = &input.ty;
        quote! { chassis::Key::for_type::<#ty>() }
    });

    let code_metafn = quote! {
        pub fn #metafn_name() -> (String, std::vec::Vec<chassis::Key>, chassis::Key) {
            (
                #userfn_name_str.into(),
                vec![ #(#dep_keys),* ],
                chassis::Key::for_type::<Self>(),
            )
        }
    };

    let code_injectfn = if return_self {
        quote! {
            pub fn #injectfn_name(__sl__: &chassis::Injector) -> Self {
                Self::#userfn_name(#(#resolves),*)
            }
        }
    } else {
        let rty = &sig.output.ty;
        let body = quote! { Self::#userfn_name(#(#resolves),*) };
        let fn_sig = quote! { pub fn #injectfn_name(__sl__: &chassis::Injector) };

        match &sig.output.wrapper {
            Some(WrapperType::Arc) => quote! { #fn_sig -> Arc<#rty> { #body } },
            Some(WrapperType::Box) => quote! { #fn_sig -> Box<#rty> { #body } },
            None => quote! { #fn_sig -> #rty { #body } },
        }
    };

    quote! {
        #code_metafn

        #code_injectfn
    }
}
