extern crate proc_macro;
extern crate syn;
#[macro_use] extern crate quote;

use proc_macro::{TokenStream};
use syn::{FnArg, Type, Ident, ReturnType};

struct InjectFnArg {
    name: Ident,
    ty: Type,
}

struct InjectFnSig {
    name: Ident,
    inputs: Vec<InjectFnArg>,
    output: Type,
}

#[proc_macro_attribute]
pub fn inject(args: TokenStream, input: TokenStream) -> TokenStream {
    let function: syn::ItemFn = syn::parse(input).unwrap();
    codegen(parse_sig(function))
}

fn parse_sig(function: syn::ItemFn) -> InjectFnSig {
    let mut inputs = Vec::new();
    for input in &function.sig.inputs {
        let (ident, ty) = match input {
            syn::FnArg::Typed(arg) => match *arg.pat {
                syn::Pat::Ident(ref pat) => (&pat.ident, &arg.ty),
                _ => panic!("invalid use of pattern")
            }
            _ => unreachable!("only usable on functions"),
        };

        inputs.push(InjectFnArg { name : ident.clone(), ty: *ty.clone() });
    }

    let rty: Type = match function.sig.output {
        syn::ReturnType::Default => panic!("return type required"),
        syn::ReturnType::Type(_, ty) => *ty,
    };

    InjectFnSig {
        name: function.sig.ident,
        inputs,
        output: rty
    }
}

fn codegen(sig: InjectFnSig) -> TokenStream {
    let name = sig.output;

    let code = quote! {
        impl Factory<()> for #name {
            fn create(sl: &chassis::ServiceLocator) -> Arc<()> {
                Arc::new(())
            }
        }
    };
    code.into()

/*    quote! {
        impl Factory<#rty> for #name {
            fn create(sl: &chassis::ServiceLocator) -> Arc<#rty> {
                Arc::new(#rty(#inputs))
            }
        }
    }*/
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
