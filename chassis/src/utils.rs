use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;

pub fn to_tokens(x: &impl ToTokens) -> TokenStream2 {
    let mut tokens = TokenStream2::new();
    x.to_tokens(&mut tokens);
    tokens
}
