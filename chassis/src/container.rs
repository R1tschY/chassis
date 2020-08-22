use std::collections::HashMap;
use std::fmt;

use crate::errors::{ChassisError, ChassisResult};
use crate::model::{Implementation, Module, StaticKey};
use syn::spanned::Spanned;

pub struct IocContainer {
    bindings: HashMap<StaticKey, Implementation>,
}

impl IocContainer {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }

    pub fn resolve(&self, key: &StaticKey) -> Option<&Implementation> {
        self.bindings.get(key)
    }

    pub fn add(&mut self, key: StaticKey, implementation: Implementation) -> ChassisResult<()> {
        let rty = implementation.rty.clone();
        let other = self.bindings.insert(key.clone(), implementation);
        if let Some(other) = other {
            return Err(ChassisError::DuplicateImplementation(
                key.to_string(),
                rty.span(),
                other.rty.span(),
            ));
        }
        Ok(())
    }

    pub fn add_module(&mut self, module: Module) -> ChassisResult<()> {
        for binding in module.bindings {
            self.add(binding.key, binding.implementation)?;
        }
        Ok(())
    }
}

impl fmt::Debug for IocContainer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("IocContainer")
            .field(&self.bindings.keys().cloned().collect::<Vec<StaticKey>>())
            .finish()
    }
}
