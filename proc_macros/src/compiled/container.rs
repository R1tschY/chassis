use crate::compiled::{Implementation, Module, StaticKey};
use std::collections::HashMap;
use std::fmt;

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

    pub fn add(&mut self, key: StaticKey, implementation: Implementation) {
        let old = self.bindings.insert(key, implementation);
        if let Some(old) = old {
            panic!("Already implementation existing");
        }
    }

    pub fn add_module(&mut self, module: Module) {
        for binding in module.bindings {
            self.add(binding.key, binding.implementation);
        }
    }
}

impl fmt::Debug for IocContainer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("IocContainer")
            .field(&self.bindings.keys().cloned().collect::<Vec<StaticKey>>())
            .finish()
    }
}
