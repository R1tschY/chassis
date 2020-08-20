use proc_macro::TokenStream;

#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

mod arguments;
mod attributes;
mod container;
mod diagnostic;
mod parse;
mod signature;
mod syn_ext;
mod utils;

#[proc_macro_attribute]
pub fn integration(args: TokenStream, input: TokenStream) -> TokenStream {
    crate::parse::integration(args, input)
}
