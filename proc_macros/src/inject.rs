use proc_macro::TokenStream;

use proc_macro2::{Ident, Span};
use syn::parse::{Parse, ParseStream};
use syn::Expr;

use crate::attributes::get_annotation_attribute;
use crate::signature::{process_sig, InjectFn, WrapperType};
use crate::syn_ext::IdentExt;
use syn::export::TokenStream2;

pub const INJECT_META_PREFIX: &str = "__injectbind_";
pub const INJECT_PREFIX: &str = "__inject_";

struct KeyAttributeMeta(Expr);

impl Parse for KeyAttributeMeta {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        parenthesized!(content in input);
        let expr: Expr = content.parse()?;
        Ok(Self(expr))
    }
}

pub fn factory(input: TokenStream) -> TokenStream {
    let mut path: syn::Path = syn::parse(input).unwrap();

    let mut last_seg = path
        .segments
        .last_mut()
        .expect("expected a path with at least one segment");
    last_seg.ident = last_seg.ident.prepend(INJECT_PREFIX);

    (quote! { #path }).into()
}

pub fn inject(input: TokenStream) -> TokenStream {
    let mut function: syn::ImplItemMethod = parse_macro_input!(input as syn::ImplItemMethod);
    let sig: InjectFn = process_sig(&mut function);
    codegen_classfn(Span::call_site(), &function, sig)
}

fn codegen_classfn(span: Span, userfn: &syn::ImplItemMethod, sig: InjectFn) -> TokenStream {
    let injectfns = codegen_injectfns(span, &sig, true);

    let code = quote! {
        #userfn

        #injectfns
    };
    code.into()
}

pub fn codegen_injectfns(
    span: Span,
    sig: &InjectFn,
    return_self: bool,
) -> proc_macro2::TokenStream {
    let userfn_name = &sig.name;
    let userfn_name_str = sig.name.to_string();
    let injectfn_name = userfn_name.prepend(INJECT_PREFIX);
    let metafn_name = userfn_name.prepend(INJECT_META_PREFIX);

    let mut dep_keys: Vec<proc_macro2::TokenStream> = vec![];
    for input in &sig.inputs {
        let ty = &input.ty.outer_ty;
        let tokens = if let Some(annotation) = get_annotation_attribute(&input.attrs) {
            let KeyAttributeMeta(expr) = syn::parse2(annotation.tokens.clone()).unwrap();
            quote! {
                chassis::TypedKey::< <#ty as chassis::ResolveInto>::Item >
                    ::new_with_annotation(#expr)
            }
        } else {
            quote! { chassis::TypedKey::< <#ty as chassis::ResolveInto>::Item >::new() }
        };
        dep_keys.push(tokens);
    }

    let resolves = dep_keys.iter().map(|key| {
        quote! { __sl__.resolve_to(#key) }
    });

    let factory = match &sig.output.wrapper {
        Some(WrapperType::Arc) => "to_arc_factory",
        Some(WrapperType::Box) => "to_box_factory",
        None => "to_factory",
    };
    let factory_ident = Ident::new(factory, Span::call_site());
    let rty_token = if return_self {
        quote! { Self }
    } else {
        let rty = &sig.output.inner_ty;
        quote! { #rty }
    };
    let annotation = if let Some(annotation) = get_annotation_attribute(&sig.attrs) {
        let KeyAttributeMeta(expr) = syn::parse2(annotation.tokens.clone()).unwrap();
        quote! { binding.annotated_with(#expr); }
    } else {
        TokenStream2::new()
    };

    let code_metafn = quote_spanned! {span=>
        pub fn #metafn_name(__binder__: &mut chassis::Binder) {
            #[allow(unused_imports)] use chassis::Named;

            let mut binding = __binder__.bind::<#rty_token>();
            #annotation
            binding.#factory_ident(
                    Self::#injectfn_name,
                    chassis::meta::InjectionPoint::for_module_function(
                        #userfn_name_str,
                        &[ #(#dep_keys.into()),* ],
                )
            )
        }
    };

    let code_injectfn = if return_self {
        quote! {
            pub fn #injectfn_name(__sl__: &chassis::Injector) -> Self {
                Self::#userfn_name(#(#resolves),*)
            }
        }
    } else {
        let rty = &sig.output.outer_ty;
        let body = quote! { Self::#userfn_name(#(#resolves),*) };
        let fn_sig = quote! { pub fn #injectfn_name(__sl__: &chassis::Injector) };
        quote! { #fn_sig -> #rty { #body } }
    };

    quote! {
        #code_metafn

        #code_injectfn
    }
}
