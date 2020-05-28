use crate::config::dependency::Dependency;
use crate::{Key};
use std::any::TypeId;

pub struct InjectionPoint {
    member: &'static str,
    declaring_type: Option<TypeId>,
    dependencies: Vec<Dependency>,
}

impl InjectionPoint {
    pub fn for_module_function(member: &'static str, parameters: &[Key]) -> Self {
        Self {
            member,
            declaring_type: None,
            dependencies: parameters
                .iter()
                .enumerate()
                .map(|(i, key)| Dependency::new(key.clone(), i as i8))
                .collect(),
        }
    }

    pub fn member(&self) -> &'static str {
        &self.member
    }

    pub fn declaring_type(&self) -> Option<TypeId> {
        self.declaring_type.clone()
    }

    pub fn dependencies(&self) -> &[Dependency] {
        &self.dependencies
    }
}
