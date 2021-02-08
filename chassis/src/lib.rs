#![doc(html_root_url = "https://docs.rs/chassis/0.2.0")]
#![cfg_attr(nightly_diagnostics, feature(proc_macro_diagnostic, proc_macro_span))]

//! Compile-time dependency injector.
//!
//! *Let the compiler generate your dependency injection code.*
//!
//! ## Goals
//! * Detect errors at compile time like missing dependencies or cyclic dependencies
//! * No need to annotate your classes (support for third-party classes)
//! * No required usage of `std::sync::Arc`
//! * Zero overhead: Fast as hand-written code
//!     * No use of runtime type information ([`Any`])
//!
//! [`Any`]: std::any::Any
//!
//! ## Use
//!
//! Add `chassis` to your crate dependencies
//! ```toml
//! [dependencies]
//! chassis = "^0.2.0"
//! ```
//!
//! Create a module for your dependency injection logic and annotate it with
//! `#[chassis::integration]`. The code in this module will be inspected by this attribute.
//! ```rust,no_run
//! #[chassis::integration]
//! mod integration {
//!    use super::*;
//!
//!    // ...
//! }
//! ```
//!
//! Structs will be modules that can provide dependencies with functions
//! and that itself can have dependencies.
//! *Note: Currently only associated functions are supported!*
//! ```rust,no_run
//! # pub struct Dependency1;
//! # pub struct Dependency2;
//! # pub struct Dependency3;
//! # impl Dependency3 { fn new(dep1: Dependency1, dep2: Dependency2) -> Self { Self } }
//! pub struct Module;
//! impl Module {
//!     pub fn provide_something(dep1: Dependency1, dep2: Dependency2) -> Dependency3 {
//!         Dependency3::new(dep1, dep2)
//!     }
//!     // ...
//! }
//! ```
//!
//! Traits will be components. For each trait a implemented component will be created. The generated
//! implementation will have a `Impl` suffix, for example `ComponentImpl`. Also a
//! `ComponentImpl::new` function is created.
//! ```rust,no_run
//! # pub struct MainClass;
//! pub trait Component {
//!     fn resolve_main_class(&self) -> MainClass;
//! }
//! ```
//!
//! ## Example
//! ```rust,no_run
//! use std::rc::Rc;
//!
//! // define your business logic
//!
//! /// printer trait
//! pub trait Printer {
//!     fn print(&self, input: &str);
//! }
//!
//! /// a printer implementation
//! pub struct StdoutPrinter;
//! impl Printer for StdoutPrinter {
//!     fn print(&self, input: &str) {
//!         println!("{}", input);
//!     }
//! }
//!
//! /// greeter for messages
//! pub struct Greeter {
//!     message: String,
//!     printer: Rc<dyn Printer>,
//! }
//! impl Greeter {
//!     /// constructor with dependencies
//!     pub fn new(message: String, printer: Rc<dyn Printer>) -> Self {
//!         Self { message, printer }
//!     }
//!
//!     /// your business logic
//!     pub fn say_hello(&self) {
//!         self.printer.print(&self.message);
//!     }
//! }
//!
//! /// module that is parsed to create the dependency injection code
//! #[chassis::integration]
//! mod integration {
//!     use super::*;
//!
//!     pub struct DemoModule;
//!
//!     // use strong types when in need to distinguish
//!     pub struct Message(String);
//!
//!     /// Define how to create your dependencies
//!     impl DemoModule {
//!         pub fn provide_printer() -> Rc<dyn Printer> {
//!             Rc::new(StdoutPrinter)
//!         }
//!
//!         pub fn provide_message() -> Message {
//!             Message("Hello World".to_string())
//!         }
//!
//!         pub fn provide_greeter(
//!             message: Message,
//!             printer: Rc<dyn Printer>
//!         ) -> Greeter {
//!             Greeter::new(message.0, printer)
//!         }
//!     }
//!
//!     /// Define which dependencies you need.
//!     ///
//!     /// A struct `DemoComponentImpl` will be created for
//!     /// you which implements `DemoComponent`.
//!     pub trait DemoComponent {
//!         /// request the to create injection code for our main class `Greeter`
//!         fn resolve_greeter(&self) -> Greeter;
//!     }
//! }
//!
//! fn main() {
//!     // import component trait
//!     use crate::integration::DemoComponent;
//!
//!     // use generated component implementation
//!     let injector = integration::DemoComponentImpl::new();
//!
//!     // Resolve main dependency
//!     // Note: it can not fail at runtime!
//!     let greeter = injector.resolve_greeter();
//!
//!     // enjoy!
//!     greeter.say_hello();
//! }
//! ```
//!
//! The generated implementation will roughly look like this
//! (*Tip: use [cargo-expand] to inspect the code*):
//! ```rust,no_run
//! # struct Greeter;
//! # struct Message;
//! # struct Printer;
//! # struct DemoModule;
//! # trait DemoComponent {
//! #   fn resolve_greeter(&self) -> Greeter;
//! # }
//! # impl DemoModule {
//! #     pub fn provide_printer() -> Printer {
//! #         Printer
//! #     }
//! #     pub fn provide_message() -> Message {
//! #         Message
//! #     }
//! #     pub fn provide_greeter(
//! #         message: Message,
//! #         printer: Printer
//! #     ) -> Greeter {
//! #         Greeter
//! #     }
//! # }
//! pub struct DemoComponentImpl{}
//!
//! impl DemoComponentImpl {
//!     pub fn new() -> Self { Self {} }
//! }
//!
//! impl DemoComponent for DemoComponentImpl {
//!     fn resolve_greeter(&self) -> Greeter {
//!         DemoModule::provide_greeter(
//!             DemoModule::provide_message(),
//!             DemoModule::provide_printer())
//!     }
//! }
//! ```
//!
//! [cargo-expand]: https://crates.io/crates/cargo-expand
//!
//! ## Limitations
//! * Dependencies are looked up through the syntax token
//!     * `Rc<Dep>` and `Rc< Dep >` are the same
//!     * but `Rc<Dep>` and `Rc<crate::Dep>` never
//!     * Also types aliases with `type` result in different type keys
//! * Currently lifetimes in the types are not supported (also `'static`)
//! * Currently generics are not handeled correctly
//! * Currently only the first error is show at compile time
//! * Currently modules can not have `&self`-methods, so inner data is useless
//! * Currently it is not possible to request a reference to a registered non-reference type
//!     * Like `&MyType` when `MyType` is provided by a module
//!
//! ## Singletons
//!
//! Normaly for every needed dependency the provider function on the module is called. This results
//! in types created multiple times. This is maybe not intended. The solution is to use a
//! `singleton` attribute. The provide method will than only called once at build time of the
//! component (call to `ComponentImpl::new`). The requirement is that the type implements the
//! [`Clone`] trait. It is recommendable to use a shared reference type like [`Rc`] or [`Arc`] for
//! singletons so that really only one instance is created.
//!
//! ### Example
//! ```rust,no_run
//! # #[chassis::integration]
//! # mod integration {
//! #   use std::rc::Rc;
//! #   trait Printer {}
//! #   struct StdoutPrinter;
//! #   impl Printer for StdoutPrinter {}
//! #   struct Module;
//! impl Module {
//!     #[singleton]
//!     pub fn provide_printer() -> Rc<dyn Printer> {
//!         Rc::new(StdoutPrinter)
//!     }
//! }
//! # }
//! ```
//!
//! [`Clone`]: std::clone::Clone
//! [`Copy`]: std::marker::Copy
//! [`Rc`]: std::rc::Rc
//! [`Arc`]: std::sync::Arc

#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use proc_macro::TokenStream;

use proc_macro2::TokenStream as TokenStream2;
use syn::spanned::Spanned;

use crate::codegen::codegen_component_impl;
use crate::container::IocContainer;
use crate::errors::{codegen_errors, ChassisError, ChassisResult};
use crate::parse::parse_block;

mod codegen;
mod container;
mod diagnostic;
mod errors;
mod key;
mod model;
mod parse;
mod syn_ext;
mod utils;

/// Attribute for modules
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
                mod_block.span(),
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
