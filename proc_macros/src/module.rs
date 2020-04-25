use proc_macro::TokenStream;

use crate::inject::{codegen_injectfns, INJECT_PREFIX};
use crate::sig::{parse_sig, InjectFn};
use crate::syn_ext::IdentExt;

pub fn module(input: TokenStream) -> TokenStream {
    // parse
    let impl_block: syn::ItemImpl = syn::parse(input).unwrap();
    assert!(
        impl_block.trait_.is_none(),
        "macro cannot applied to trait impl blocks"
    );

    let name: &syn::Type = &impl_block.self_ty;
    let mut functions: Vec<InjectFn> = vec![];
    for item in &impl_block.items {
        if let syn::ImplItem::Method(method) = item {
            if let syn::Visibility::Public(_) = method.vis {
                functions.push(parse_sig(&method.sig));
            }
            //method.attrs.contains(syn::Attribute)
        }
    }

    // codegen
    let inject_fns: Vec<_> = functions
        .iter()
        .map(|function| codegen_injectfns(function, false))
        .collect();
    let bindings: Vec<_> = functions
        .iter()
        .map(|function| {
            let inject_fn = function.name.prepend(INJECT_PREFIX);
            quote! { __sl__.register(FactoryLoader::new(Self::#inject_fn)); }
        })
        .collect();

    (quote! {
        #impl_block

        impl #name {
            #(#inject_fns)*
        }

        impl chassis::Module for #name {
            fn configure(&self, __sl__: &mut chassis::Injector) {
                #(#bindings)*
            }
        }
    })
    .into()
}
