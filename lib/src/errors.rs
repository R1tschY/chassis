use crate::Key;
use std::fmt;

#[derive(Debug)]
pub enum ChassisError {
    MissingImplementation(Key),
    // CyclicDependency(Vec<Dependency>),
    // CreateError(Box<dyn Error>),
}

pub struct Errors {
    errors: Vec<ChassisError>,
}

impl Errors {
    pub fn new() -> Self {
        Self { errors: vec![] }
    }

    pub fn add(&mut self, error: ChassisError) {
        self.errors.push(error)
    }

    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }
}

impl fmt::Debug for Errors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(&self.errors).finish()
    }
}

pub type ChassisResult<T> = Result<T, Errors>;
