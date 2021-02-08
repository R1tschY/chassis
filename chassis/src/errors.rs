use std::fmt;

use proc_macro2::Span;
use proc_macro2::TokenStream as TokenStream2;

use crate::diagnostic::{Diagnostic, DiagnosticExt};

#[derive(Debug)]
pub enum ChassisError {
    InternalError(String),
    IllegalInput(String, Span),
    CyclicDependency(Vec<(String, Span)>),
    MissingDependency(Vec<(String, Span)>),
    DuplicateImplementation(String, Span, Span),
}

pub type ChassisResult<T> = Result<T, ChassisError>;

impl From<fmt::Error> for ChassisError {
    fn from(_: fmt::Error) -> Self {
        ChassisError::InternalError("Format error".into())
    }
}

pub fn codegen_errors(err: ChassisError) -> TokenStream2 {
    match err {
        ChassisError::InternalError(message) => Span::call_site().error(message),
        ChassisError::IllegalInput(message, span) => span.error(message),
        ChassisError::CyclicDependency(chain) => error_from_dep_chain(
            format!("Cyclic dependency for `{}`", chain[chain.len() - 1].0),
            chain,
        ),
        ChassisError::MissingDependency(chain) => error_from_dep_chain(
            format!("Missing dependency `{}`", chain[chain.len() - 1].0),
            chain,
        ),
        ChassisError::DuplicateImplementation(ty, one, two) => one
            .error(format!("Duplicate implementation for `{}`", ty))
            .help_in("Other implementation found here", two),
    }
    .emit()
}

fn error_from_dep_chain(message: String, chain: Vec<(String, Span)>) -> Diagnostic {
    let mut iter = chain.iter().rev();
    let fail = iter.next().unwrap();
    let mut diagnostic = fail.1.error(message);
    for cause in iter {
        diagnostic = diagnostic.help_in(format!("required for `{}`", cause.0), cause.1);
    }
    diagnostic
}
