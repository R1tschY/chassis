use syn::export::{ToTokens, TokenStream2};

pub fn to_tokens(x: impl ToTokens) -> TokenStream2 {
    let mut tokens = TokenStream2::new();
    x.to_tokens(&mut tokens);
    tokens
}
