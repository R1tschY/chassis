#[derive(Debug)]
pub enum ChassisError {
    // TODO: allow multiple errors and track span
    MissingReturnTypeInComponent,
    TypeItemInComponent,
    DefaultImplementationInComponent,
}

pub type ChassisResult<T> = Result<T, ChassisError>;
