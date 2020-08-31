use std::cell::RefCell;
use std::ops::Deref;

use proc_macro2::Span;

use crate::container::IocContainer;
use crate::errors::{ChassisError, ChassisResult};
use crate::key::StaticKey;
use crate::model::Implementation;

/// Environment of code generation
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CodegenEnv {
    TraitImpl,
    Ctor,
}

/// Context for generation of one provider.
///
/// helps detecting cyclic dependencies.
pub struct CodegenContext<'a> {
    container: &'a IocContainer,
    resolving: RefCell<Vec<(StaticKey, Span)>>,
    env: CodegenEnv,
}

pub struct CodegenContextScope<'a, 'b> {
    context: &'a CodegenContext<'b>,
    result: Option<&'b Implementation>,
}

impl<'a> CodegenContext<'a> {
    pub fn new(container: &'a IocContainer, env: CodegenEnv) -> Self {
        Self {
            container,
            env,
            resolving: RefCell::new(vec![]),
        }
    }

    pub fn env(&self) -> CodegenEnv {
        self.env
    }

    pub fn dependency_chain(&self) -> Vec<(String, Span)> {
        Self::dependency_chain_inner(&self.resolving.borrow())
    }

    fn dependency_chain_inner(resolving: &[(StaticKey, Span)]) -> Vec<(String, Span)> {
        resolving
            .iter()
            .map(|(k, span)| (k.to_string(), *span))
            .collect()
    }

    pub fn enter_resolving(
        &self,
        key: &StaticKey,
        span: Span,
    ) -> ChassisResult<CodegenContextScope<'_, 'a>> {
        {
            let mut resolving = self.resolving.borrow_mut();
            if resolving.iter().any(|(k, _)| k == key) {
                resolving.push((key.clone(), span));
                let req_chain = Self::dependency_chain_inner(&resolving);
                resolving.pop();
                return Err(ChassisError::CyclicDependency(req_chain));
            }
            resolving.push((key.clone(), span));
        }

        let result = self.container.resolve(key);
        Ok(CodegenContextScope {
            context: self,
            result,
        })
    }

    fn leave_resolving(&self) {
        self.resolving.borrow_mut().pop();
    }
}

impl<'a, 'b> Drop for CodegenContextScope<'a, 'b> {
    fn drop(&mut self) {
        self.context.leave_resolving();
    }
}

impl<'a, 'b> Deref for CodegenContextScope<'a, 'b> {
    type Target = Option<&'b Implementation>;

    fn deref(&self) -> &Self::Target {
        &self.result
    }
}
