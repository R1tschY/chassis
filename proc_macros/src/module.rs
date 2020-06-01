use proc_macro::TokenStream;

use crate::inject::{codegen_injectfns, INJECT_META_PREFIX};
use crate::sig::{process_sig, InjectFn};
use crate::syn_ext::IdentExt;
use proc_macro::Ident;
use std::ops::Deref;
use syn::export::TokenStream2;
use syn::{FnArg, Signature, Type};

fn require_configure_signature(sig: &Signature) -> bool {
    let mut has_receiver = false;

    // only allow &self
    for input in &sig.inputs {
        if let FnArg::Receiver(receiver) = input {
            has_receiver = true;
            if receiver.mutability.is_some() || receiver.reference.is_none() {
                // is not &self
                return false;
            }
        }
    }

    // allow only one &mut Binder argument
    if (has_receiver && sig.inputs.len() != 2) || (!has_receiver && sig.inputs.len() != 1) {
        // wrong number of arguments
        return false;
    }

    for input in &sig.inputs {
        if let FnArg::Typed(typed) = input {
            if let Type::Reference(ref_ty) = typed.ty.deref() {
                if ref_ty.mutability.is_some() {
                    // argument is not const reference
                    return false;
                }
            // TODO: check for Binder type?
            } else {
                // argument is not a reference
                return false;
            }
        }
    }

    true
}

pub fn module(input: TokenStream) -> TokenStream {
    // parse
    let mut impl_block: syn::ItemImpl = syn::parse(input).unwrap();
    assert!(
        impl_block.trait_.is_none(),
        "macro cannot applied to trait impl blocks"
    );

    let mut configure: Option<Signature> = None;
    let name: &syn::Type = &impl_block.self_ty;
    let mut functions: Vec<InjectFn> = vec![];
    for item in &mut impl_block.items {
        if let syn::ImplItem::Method(method) = item {
            if &method.sig.ident.to_string() == "configure" {
                configure = Some(method.sig.clone());
            } else if let syn::Visibility::Public(_) = method.vis {
                functions.push(process_sig(&mut method.sig));
            }
        }
    }

    // codegen
    let inject_fns: Vec<_> = functions
        .iter()
        .map(|function| codegen_injectfns(function.name.span(), function, false))
        .collect();
    let bindings: Vec<_> = functions
        .iter()
        .map(|function| {
            let meta_fn = function.name.prepend(INJECT_META_PREFIX);
            quote! {
                Self::#meta_fn(__binder__);
            }
        })
        .collect();
    let user_configure = if let Some(sig) = configure {
        if !require_configure_signature(&sig) {
            panic!(
                "The signature of the configure function must be \
                 `fn configure(&self, binder: &Binder)` or `fn configure(binder: &Binder)`"
            );
        }
        if sig.receiver().is_some() {
            quote! { self.configure(__binder__); }
        } else {
            quote! { Self::configure(__binder__); }
        }
    } else {
        TokenStream2::new()
    };

    (quote! {
        #impl_block

        impl #name {
            #(#inject_fns)*
        }

        impl chassis::Module for #name {
            fn configure(&self, __binder__: &mut chassis::Binder) {
                #(#bindings)*
                #user_configure
            }
        }
    })
    .into()
}
