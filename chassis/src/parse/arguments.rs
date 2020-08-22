use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;

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

        let name = input.parse()?;
        let assign_token = input.parse()?;
        let bracket_token = bracketed!(content in input);
        let value = content.parse_terminated(syn::Path::parse)?;
        Ok(ComponentAttrArg {
            name,
            assign_token,
            bracket_token,
            value,
        })
    }
}
