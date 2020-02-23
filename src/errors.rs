use std::error::Error;

pub enum ChassisError {
    NotFound,
    CyclicDependency,
    CreateError(Box<dyn Error>)
}

type ChassisResult<T> = Result<T, ChassisError>;