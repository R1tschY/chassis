use crate::container::IocContainer;
use crate::parse::{Implementation, StaticKey};
use std::cell::RefCell;
use std::ops::Deref;

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

    pub fn enter_resolving(&self, key: &StaticKey) -> CodegenContextScope<'_, 'a> {
        {
            let mut resolving = self.resolving.borrow_mut();
            if resolving.contains(key) {
                resolving.push(key.clone());
                let req_chain = resolving
                    .iter()
                    .map(|k| k.to_string())
                    .collect::<Vec<String>>()
                    .join(" -> ");
                panic!("Cyclic dependency found: {}", req_chain);
            }
            resolving.push(key.clone());
        }

        let result = self.container.resolve(key);
        CodegenContextScope {
            context: self,
            result,
        }
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
