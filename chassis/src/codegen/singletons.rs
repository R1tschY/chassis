use crate::codegen::context::CodegenContext;
use crate::codegen::singletons;
use crate::container::IocContainer;
use crate::errors::{ChassisError, ChassisResult};
use crate::model::{ComponentTrait, Request, StaticKey};
use std::collections::HashSet;
use std::ops::Deref;

pub const SINGLETON_FIELD_PREFIX: &str = "singleton_of_";

fn singletons_for_provider(
    request: &Request,
    container: &IocContainer,
    singletons: &mut HashSet<StaticKey>,
) -> ChassisResult<()> {
    let provider_ctx = CodegenContext::new(container);
    singletons_for_key(&request.key, &provider_ctx, singletons)
}

fn singletons_for_key(
    key: &StaticKey,
    ctx: &CodegenContext,
    singletons: &mut HashSet<StaticKey>,
) -> ChassisResult<()> {
    let scope = ctx.enter_resolving(key)?;

    if singletons.contains(key) {
        return Ok(());
    }

    if let Some(implementation) = scope.deref() {
        implementation
            .injection_point
            .deps
            .iter()
            .map(|dep| singletons_for_key(&dep.key, ctx, singletons))
            .collect::<ChassisResult<()>>()?;

        if implementation.singleton {
            singletons.insert(key.clone());
        }

        Ok(())
    } else {
        Err(ChassisError::MissingDependency(ctx.dependency_chain()))
    }
}

pub fn find_singletons(
    component: &ComponentTrait,
    container: &IocContainer,
) -> ChassisResult<HashSet<StaticKey>> {
    let mut singletons: HashSet<StaticKey> = HashSet::new();
    component
        .requests
        .iter()
        .map(|request| singletons::singletons_for_provider(request, container, &mut singletons))
        .collect::<ChassisResult<()>>()?;
    Ok(singletons)
}
