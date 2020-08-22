use std::ops::Deref;

use crate::codegen::context::CodegenContext;
use crate::container::IocContainer;
use crate::errors::{ChassisError, ChassisResult};
use crate::model::{ComponentTrait, Implementation, Request, StaticKey};
use crate::syn_ext::IdentExt;
use syn::export::TokenStream2;

mod context;

pub fn codegen_component_impl(
    component: ComponentTrait,
    container: &IocContainer,
) -> ChassisResult<TokenStream2> {
    let trait_name = component.trait_name;
    let impl_name = trait_name.append("Impl");
    let impl_items = component
        .requests
        .into_iter()
        .map(|request| codegen_provider_fn(request, container))
        .collect::<ChassisResult<Vec<_>>>()?;

    Ok(quote! {
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
    })
}

/// Creates function for trait implementation
fn codegen_provider_fn(request: Request, container: &IocContainer) -> ChassisResult<TokenStream2> {
    let provider_ctx = CodegenContext::new(container);

    let impl_code = codegen_for_key(&request.key, &provider_ctx)?;
    let rty = &request.key.type_(); // TODO: clone with call_span
    let name = &request.name;
    let span = request.name.span(); // TODO: use Signature as span
    Ok(quote_spanned! {span=>
        fn #name(&self) -> #rty {
            #impl_code
        }
    })
}

/// Creates expression for getting `key`
fn codegen_for_key(key: &StaticKey, ctx: &CodegenContext) -> ChassisResult<TokenStream2> {
    let scope = ctx.enter_resolving(key)?;

    if let Some(binding) = scope.deref() {
        codegen_impl(binding, ctx)
    } else {
        Err(ChassisError::MissingDependency(ctx.dependency_chain()))
    }
}

/// Creates expression for `implementation`
fn codegen_impl(
    implementation: &Implementation,
    ctx: &CodegenContext,
) -> ChassisResult<TokenStream2> {
    let module = &implementation.module;
    let func = &implementation.func;
    let dep_impls = implementation
        .injection_point
        .deps
        .iter()
        .map(|dep| codegen_for_key(&dep.key, ctx))
        .collect::<ChassisResult<Vec<TokenStream2>>>()?;

    Ok(quote! {
        #module::#func(#(#dep_impls),*)
    })
}
