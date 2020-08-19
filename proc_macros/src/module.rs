use proc_macro::TokenStream;
use std::ops::Deref;

use syn::export::TokenStream2;
use syn::spanned::Spanned;
use syn::{FnArg, Signature, Type};

use crate::diagnostic::{Diagnostic, DiagnosticExt};
use crate::inject::{codegen_injectfns, INJECT_META_PREFIX};
use crate::signature::{process_sig, InjectFn};
use crate::syn_ext::IdentExt;
use crate::utils::to_tokens;

fn error_configure_signature(sig: &Signature) -> Diagnostic {
    sig.span().error(format!(
        "The signature of the configure function must be \
         `fn configure(&self, binder: &mut Binder)` or \
         `fn configure(binder: &mut Binder)`, \
         got `{}`",
        to_tokens(sig)
    ))
}

fn is_valid_configure_signature(sig: &Signature) -> Option<Diagnostic> {
    let mut has_receiver = false;

    // only allow &self
    for input in &sig.inputs {
        if let FnArg::Receiver(receiver) = input {
            has_receiver = true;
            if receiver.mutability.is_some() || receiver.reference.is_none() {
                return Some(
                    error_configure_signature(sig)
                        .help_in("receiver is not `&self`", receiver.span()),
                );
            }
        }
    }

    // allow only one &mut Binder argument
    if (has_receiver && sig.inputs.len() != 2) || (!has_receiver && sig.inputs.len() != 1) {
        // wrong number of arguments
        return Some(error_configure_signature(sig).help("wrong number of arguments"));
    }

    for input in &sig.inputs {
        if let FnArg::Typed(typed) = input {
            if let Type::Reference(ref_ty) = typed.ty.deref() {
                // TODO: check for Binder type?
                if ref_ty.mutability.is_none() {
                    return Some(
                        error_configure_signature(sig)
                            .help_in("argument is not mutable reference", ref_ty.span()),
                    );
                }
            } else {
                return Some(
                    error_configure_signature(sig)
                        .help_in("argument is not a reference", typed.span()),
                );
            }
        }
    }

    None
}

pub fn module(input: TokenStream) -> TokenStream {
    // parse
    let mut impl_block: syn::ItemImpl = parse_macro_input!(input);
    assert!(
        impl_block.trait_.is_none(),
        "macro cannot applied to trait impl blocks"
    );

    let mut configure: Option<Signature> = None;
    let name: &syn::Type = &impl_block.self_ty;
    let mut functions: Vec<InjectFn> = vec![];
    for mut item in &mut impl_block.items {
        if let syn::ImplItem::Method(ref mut method) = &mut item {
            if &method.sig.ident.to_string() == "configure" {
                configure = Some(method.sig.clone());
            } else if let syn::Visibility::Public(_) = method.vis {
                functions.push(process_sig(method));
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
        if let Some(diag) = is_valid_configure_signature(&sig) {
            diag.emit()
        } else if sig.receiver().is_some() {
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
