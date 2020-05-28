#![feature(proc_macro_diagnostic)]

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use proc_macro::TokenStream;

mod syn_ext;

mod inject;
mod module;
mod sig;

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
