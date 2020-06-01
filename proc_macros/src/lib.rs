#![cfg_attr(nightly_diagnostics, feature(proc_macro_diagnostic, proc_macro_span))]

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use proc_macro::TokenStream;

mod syn_ext;

mod diagnostic;
mod inject;
mod module;
mod signature;

#[proc_macro]
pub fn factory(input: TokenStream) -> TokenStream {
    crate::inject::factory(input)
}

#[proc_macro_attribute]
pub fn inject(_args: TokenStream, input: TokenStream) -> TokenStream {
    crate::inject::inject(input)
}

#[proc_macro_attribute]
pub fn module(_args: TokenStream, input: TokenStream) -> TokenStream {
    crate::module::module(input)
}
