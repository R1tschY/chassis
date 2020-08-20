use crate::Key;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ChassisError {
    MissingImplementation(Key),
    // CyclicDependency(Vec<Dependency>),
    // CreateError(Box<dyn-chassis Error>),
}

#[derive(Default)]
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

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: do better
        f.debug_list().entries(&self.errors).finish()
    }
}

impl fmt::Debug for Errors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(&self.errors).finish()
    }
}

impl Error for Errors {}

pub type ChassisResult<T> = Result<T, Errors>;
