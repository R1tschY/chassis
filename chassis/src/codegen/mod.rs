use std::ops::Deref;

use proc_macro2::TokenStream;

use crate::codegen::context::CodegenContext;
use crate::container::IocContainer;
use crate::model::{ComponentTrait, Implementation, Request, StaticKey};
use crate::syn_ext::IdentExt;

mod context;

pub fn codegen_component_impl(component: ComponentTrait, container: &IocContainer) -> TokenStream {
    let trait_name = component.trait_name;
    let impl_name = trait_name.append("Impl");
    let impl_items: Vec<_> = component
        .requests
        .into_iter()
        .map(|request| codegen_provider_fn(request, container))
        .collect();

    let tokens = quote! {
        pub struct #impl_name;  // TODO: use visibility of trait

        impl #impl_name {
            pub fn new() -> Self { Self }
        }

        impl #trait_name for #impl_name {
            #(#impl_items)*
        }

        impl Default for #impl_name {
            fn default() -> Self {
                Self::new()
            }
        }
    };
    tokens
}

/// Creates function for trait implementation
fn codegen_provider_fn(request: Request, container: &IocContainer) -> TokenStream {
    let provider_ctx = CodegenContext::new(container);

    let impl_code = codegen_for_key(&request.key, &provider_ctx);
    let rty = &request.key.type_();
    let name = &request.name;
    let span = request.name.span(); // TODO: use Signature as span
    quote_spanned! {span=>
        fn #name(&self) -> #rty {
            #impl_code
        }
    }
}

/// Creates expression for getting `key`
fn codegen_for_key(key: &StaticKey, ctx: &CodegenContext) -> TokenStream {
    let scope = ctx.enter_resolving(key);

    let binding = if let Some(binding) = scope.deref() {
        binding
    } else {
        panic!("Missing binding for `{}` to resolve `TODO`", key);
    };

    codegen_impl(binding, ctx)
}

/// Creates expression for `implementation`
fn codegen_impl(implementation: &Implementation, ctx: &CodegenContext) -> TokenStream {
    match implementation {
        Implementation::Factory {
            rty: _,
            module,
            func,
            injection_point,
        } => {
            let dep_impls: Vec<TokenStream> = injection_point
                .deps
                .iter()
                .map(|dep| codegen_for_key(&dep.key, ctx))
                .collect();

            quote! {
                #module::#func(#(#dep_impls),*)
            }
        }
        Implementation::Linked(key) => codegen_for_key(key, ctx),
    }
}
