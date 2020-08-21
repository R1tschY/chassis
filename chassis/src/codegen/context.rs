use std::cell::RefCell;
use std::ops::Deref;

use crate::container::IocContainer;
use crate::errors::{ChassisError, ChassisResult};
use crate::model::{Implementation, StaticKey};
use proc_macro2::Span;
use syn::spanned::Spanned;

/// Context for generation of one provider.
///
/// helps detecting cyclic dependencies.
pub struct CodegenContext<'a> {
    container: &'a IocContainer,
    resolving: RefCell<Vec<StaticKey>>,
}

pub struct CodegenContextScope<'a, 'b> {
    context: &'a CodegenContext<'b>,
    result: Option<&'b Implementation>,
}

impl<'a> CodegenContext<'a> {
    pub fn new(container: &'a IocContainer) -> Self {
        Self {
            container,
            resolving: RefCell::new(vec![]),
        }
    }

    pub fn dependency_chain(&self) -> Vec<(String, Span)> {
        Self::dependency_chain_inner(&self.resolving.borrow())
    }

    fn dependency_chain_inner(resolving: &Vec<StaticKey>) -> Vec<(String, Span)> {
        resolving
            .iter()
            .map(|k| (k.to_string(), k.type_().span().clone()))
            .collect()
    }

    pub fn enter_resolving(&self, key: &StaticKey) -> ChassisResult<CodegenContextScope<'_, 'a>> {
        {
            let mut resolving = self.resolving.borrow_mut();
            if resolving.contains(key) {
                resolving.push(key.clone());
                let req_chain = Self::dependency_chain_inner(&resolving);
                resolving.pop();
                return Err(ChassisError::CyclicDependency(req_chain));
            }
            resolving.push(key.clone());
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
