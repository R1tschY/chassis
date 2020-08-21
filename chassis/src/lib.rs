#![cfg_attr(nightly_diagnostics, feature(proc_macro_diagnostic, proc_macro_span))]

#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use proc_macro::TokenStream;

use syn::export::TokenStream2;

use crate::codegen::codegen_component_impl;
use crate::container::IocContainer;
use crate::errors::{codegen_errors, ChassisError, ChassisResult};
use crate::parse::parse_block;
use syn::spanned::Spanned;

mod codegen;
mod container;
mod diagnostic;
mod errors;
mod model;
mod parse;
mod syn_ext;
mod utils;

#[proc_macro_attribute]
pub fn integration(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mod_block: syn::ItemMod = parse_macro_input!(input);

    match parse_integration(_args, mod_block) {
        Ok(tokens) => tokens.into(),
        Err(err) => codegen_errors(err).into(),
    }
}

fn parse_integration(
    _args: TokenStream,
    mut mod_block: syn::ItemMod,
) -> ChassisResult<TokenStream2> {
    let mut mod_impl = match &mut mod_block.content {
        Some((_, items)) => items,
        None => {
            return Err(ChassisError::IllegalInput(
                "Expected module implementation when using integration attribute".to_string(),
                mod_block.span().clone(),
            ))
        }
    };

    // Parse components and modules
    let block = parse_block(&mut mod_impl)?;

    // analyse
    let modules = block.modules;
    let mut container = IocContainer::new();
    for module in modules {
        container.add_module(module)?;
    }

    // generate
    let component_impls = block
        .components
        .into_iter()
        .map(|comp| codegen_component_impl(comp, &container))
        .collect::<ChassisResult<Vec<TokenStream2>>>()?;

    // generate result
    let mod_name = &mod_block.ident;
    let mod_vis = &mod_block.vis;
    Ok(quote! {
        #mod_vis mod #mod_name {
            #(#mod_impl)*

            #(#component_impls)*
        }
    })
}
