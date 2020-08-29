use std::ops::Deref;

use syn::export::TokenStream2;

use crate::codegen::context::{CodegenContext, CodegenEnv};
use crate::codegen::generator::ComponentBuilder;
use crate::codegen::singletons::find_singletons;
use crate::container::IocContainer;
use crate::errors::{ChassisError, ChassisResult};
use crate::model::{ComponentTrait, Implementation, Request, StaticKey};
use crate::syn_ext::IdentExt;
use singletons::SINGLETON_FIELD_PREFIX;

mod context;
mod generator;
mod singletons;

const TRAIT_IMPL_SUFFIX: &str = "Impl";

pub fn codegen_component_impl(
    component: ComponentTrait,
    container: &IocContainer,
) -> ChassisResult<TokenStream2> {
    let singletons = find_singletons(&component, container)?;
    let mut component_builder = ComponentBuilder::new();

    let impl_items = component
        .requests
        .into_iter()
        .map(|request| codegen_provider_fn(request, container))
        .collect::<ChassisResult<Vec<TokenStream2>>>()?;

    // codegen singletons
    for singleton in singletons {
        let ctx = CodegenContext::new(container, CodegenEnv::Ctor);
        let binding = ctx.enter_resolving(&singleton)?.unwrap();

        let field = binding.func.prepend(SINGLETON_FIELD_PREFIX);
        let code = codegen_impl(binding, &ctx, true)?;
        component_builder.field(field, singleton.type_().clone(), code);
    }

    // codegen component
    let trait_name = component.trait_name;
    let impl_name = trait_name.append(TRAIT_IMPL_SUFFIX);
    let component_struct = component_builder.build(&impl_name);
    Ok(quote! {
        #component_struct

        impl #trait_name for #impl_name {
            #(#impl_items)*
        }
    })
}

/// Creates function for trait implementation
fn codegen_provider_fn(request: Request, container: &IocContainer) -> ChassisResult<TokenStream2> {
    let provider_ctx = CodegenContext::new(container, CodegenEnv::TraitImpl);

    let code = codegen_for_key(&request.key, &provider_ctx)?;
    let rty = &request.key.type_(); // TODO: clone with call_span
    let name = &request.name;
    let span = request.name.span(); // TODO: use Signature as span

    Ok(quote_spanned! {span=>
        fn #name(&self) -> #rty {
            #code
        }
    })
}

/// Creates expression for getting `key`
fn codegen_for_key(key: &StaticKey, ctx: &CodegenContext) -> ChassisResult<TokenStream2> {
    let scope = ctx.enter_resolving(key)?;

    if let Some(binding) = scope.deref() {
        codegen_impl(binding, ctx, false)
    } else {
        Err(ChassisError::MissingDependency(ctx.dependency_chain()))
    }
}

/// Creates expression for `implementation`
fn codegen_impl(
    implementation: &Implementation,
    ctx: &CodegenContext,
    force_impl: bool,
) -> ChassisResult<TokenStream2> {
    let module = &implementation.module;
    let func = &implementation.func;
    let dep_impls = implementation
        .injection_point
        .deps
        .iter()
        .map(|dep| codegen_for_key(&dep.key, ctx))
        .collect::<ChassisResult<Vec<TokenStream2>>>()?;

    let code = if implementation.singleton && !force_impl {
        let singleton_field = func.prepend(SINGLETON_FIELD_PREFIX);
        match ctx.env() {
            CodegenEnv::Ctor => quote! {
                #singleton_field.clone()
            },
            CodegenEnv::TraitImpl => quote! {
                self.#singleton_field.clone() // TODO: use ref when ref needed
            },
        }
    } else {
        quote! {
            #module::#func(#(#dep_impls),*)
        }
    };

    Ok(code)
}
