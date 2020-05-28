use crate::Key;

pub enum ChassisError {
    MissingImplementation(Key),
    // CyclicDependency(Vec<Dependency>),
    // CreateError(Box<dyn Error>),
}



pub struct Errors {
    errors: Vec<ChassisError>
}

impl Errors {
    pub fn new() -> Self {
        Self { errors: vec![] }
    }

    pub fn add(&mut self, error: ChassisError) {
        self.errors.push(error)
    }
}


// type ChassisResult<T> = Result<T, Errors>;
