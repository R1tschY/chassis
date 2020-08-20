use proc_macro2::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Token;
use syn::Expr;

/// attribute argument like `modules = [mymod::MyModule]`
pub struct ComponentAttrArg {
    pub name: syn::Ident,
    pub assign_token: Token![=],
    pub bracket_token: syn::token::Bracket,
    pub value: Punctuated<syn::Path, Token![,]>,
}

/// attribute arguments like `#[component(modules = [mymod::MyModule], ...)]`
pub struct ComponentAttrArgs {
    pub args: Punctuated<ComponentAttrArg, Token![,]>,
}

impl Parse for ComponentAttrArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(ComponentAttrArgs {
            args: input.parse_terminated(ComponentAttrArg::parse)?,
        })
    }
}

impl Parse for ComponentAttrArg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(ComponentAttrArg {
            name: input.parse()?,
            assign_token: input.parse()?,
            bracket_token: bracketed!(content in input),
            value: content.parse_terminated(syn::Path::parse)?,
        })
    }
}
