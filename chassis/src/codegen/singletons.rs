use std::ops::Deref;

use proc_macro2::Span;
use syn::spanned::Spanned;

use crate::codegen::context::{CodegenContext, CodegenEnv};
use crate::container::IocContainer;
use crate::errors::{ChassisError, ChassisResult};
use crate::key::StaticKey;
use crate::model::{ComponentTrait, Request};

pub const SINGLETON_FIELD_PREFIX: &str = "singleton_of_";

fn singletons_for_provider(
    request: &Request,
    container: &IocContainer,
    singletons: &mut Vec<StaticKey>,
) -> ChassisResult<()> {
    let provider_ctx = CodegenContext::new(container, CodegenEnv::TraitImpl);
    singletons_for_key(&request.key, request.ty.span(), &provider_ctx, singletons)
}

fn singletons_for_key(
    key: &StaticKey,
    span: Span,
    ctx: &CodegenContext,
    singletons: &mut Vec<StaticKey>,
) -> ChassisResult<()> {
    let scope = ctx.enter_resolving(key, span)?;

    if singletons.contains(key) {
        return Ok(());
    }

    if let Some(implementation) = scope.deref() {
        implementation
            .injection_point
            .deps
            .iter()
            .map(|dep| singletons_for_key(&dep.key, dep.span, ctx, singletons))
            .collect::<ChassisResult<()>>()?;

        if implementation.singleton {
            singletons.push(key.clone());
        }

        Ok(())
    } else {
        Err(ChassisError::MissingDependency(ctx.dependency_chain()))
    }
}

/// Find singletons recursively in `component`.
///
/// Returns list with singletons in that order, that entries do not depend on previous entries.
pub fn find_singletons(
    component: &ComponentTrait,
    container: &IocContainer,
) -> ChassisResult<Vec<StaticKey>> {
    let mut singletons: Vec<StaticKey> = Vec::new();
    component
        .requests
        .iter()
        .map(|request| singletons_for_provider(request, container, &mut singletons))
        .collect::<ChassisResult<()>>()?;
    Ok(singletons)
}
