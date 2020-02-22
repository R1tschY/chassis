extern crate proc_macro;
#[macro_use] extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

mod syn_ext;

mod inject;


#[proc_macro]
pub fn factory(input: TokenStream) -> TokenStream {
    crate::inject::factory(input)
}

#[proc_macro_attribute]
pub fn inject(_args: TokenStream, input: TokenStream) -> TokenStream {
    crate::inject::inject(input)
}