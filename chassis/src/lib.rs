#![cfg_attr(nightly_diagnostics, feature(proc_macro_diagnostic, proc_macro_span))]

#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use proc_macro::TokenStream;

mod codegen;
mod container;
mod diagnostic;
mod errors;
mod model;
mod parse;
mod syn_ext;
mod utils;

#[proc_macro_attribute]
pub fn integration(args: TokenStream, input: TokenStream) -> TokenStream {
    crate::parse::integration(args, input)
}
