use core::fmt;
use core::hash::{Hash, Hasher};
use std::fmt::Write;

use quote::ToTokens;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{GenericArgument, TraitBoundModifier, TypeParamBound};

use crate::errors::{ChassisError, ChassisResult};

/// Key to something to inject.
///
/// Used to reference dependencies. Is a normalized type reference.
#[derive(Clone)]
pub struct StaticKey {
    ty_str: String,
}

impl StaticKey {
    pub fn try_new(ty: &syn::Type) -> ChassisResult<Self> {
        let mut ty_str = String::new();
        ty.conv_to_key_str(&mut ty_str)?;
        Ok(Self { ty_str })
    }

    #[allow(dead_code)]
    pub fn type_string(&self) -> &str {
        &self.ty_str
    }
}

impl Hash for StaticKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ty_str.hash(state);
    }
}

impl PartialEq for StaticKey {
    fn eq(&self, other: &Self) -> bool {
        self.ty_str == other.ty_str
    }
}

impl Eq for StaticKey {}

impl fmt::Debug for StaticKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("StaticKey").field(&self.ty_str).finish()
    }
}

impl fmt::Display for StaticKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.ty_str)
    }
}

// trait ToKeyStr

trait ToKeyStr {
    fn conv_to_key_str(&self, f: &mut String) -> ChassisResult<()>;

    fn to_key_str(&self) -> ChassisResult<String> {
        let mut res = String::new();
        self.conv_to_key_str(&mut res)?;
        Ok(res)
    }
}

fn dump_punctuated<T: ToKeyStr, P>(
    sep: &str,
    elems: &Punctuated<T, P>,
    f: &mut String,
) -> ChassisResult<()> {
    let mut punct = false;
    for elem in elems {
        if punct {
            f.write_str(sep)?;
        } else {
            punct = true;
        }
        elem.conv_to_key_str(f)?;
    }
    Ok(())
}

impl ToKeyStr for syn::Type {
    fn conv_to_key_str(&self, f: &mut String) -> ChassisResult<()> {
        use syn::Type::*;

        match self {
            Group(group) => group.elem.conv_to_key_str(f),
            Paren(inner) => inner.elem.conv_to_key_str(f),

            Path(path) => path.conv_to_key_str(f),
            Ptr(ptr) => ptr.conv_to_key_str(f),
            TraitObject(trait_obj) => trait_obj.conv_to_key_str(f),
            Tuple(tuple) => tuple.conv_to_key_str(f),
            Reference(reference) => reference.conv_to_key_str(f),
            ImplTrait(impl_trait) => impl_trait.conv_to_key_str(f),

            // errors
            Infer(_) => Err(ChassisError::IllegalInput(
                "Infer `_` not allowed".into(),
                self.span(),
            )),
            Macro(_) => Err(ChassisError::IllegalInput(
                "Macro call not allowed".into(),
                self.span(),
            )),
            Never(_) => Err(ChassisError::IllegalInput(
                "Never not allowed".into(),
                self.span(),
            )),
            Slice(_) => Err(ChassisError::IllegalInput(
                "Unsized slice not allowed".into(),
                self.span(),
            )),
            Array(_) => Err(ChassisError::IllegalInput(
                "Arrays are not supported".into(),
                self.span(),
            )),
            _ => Err(ChassisError::IllegalInput(
                format!("Unsupported syntax `{}`", self.to_token_stream()),
                self.span(),
            )),
        }
    }
}

impl ToKeyStr for syn::TypePath {
    fn conv_to_key_str(&self, f: &mut String) -> ChassisResult<()> {
        if self.qself.is_some() {
            Err(ChassisError::IllegalInput(
                "Self-types are not allowed".into(),
                self.span(),
            ))
        } else {
            self.path.conv_to_key_str(f)
        }
    }
}

impl ToKeyStr for syn::Path {
    fn conv_to_key_str(&self, f: &mut String) -> ChassisResult<()> {
        let mut colon = self.leading_colon.is_some();
        for seg in &self.segments {
            if colon {
                f.write_str("::")?;
            } else {
                colon = true;
            }
            seg.ident.conv_to_key_str(f)?;
            seg.arguments.conv_to_key_str(f)?;
        }
        Ok(())
    }
}

impl ToKeyStr for syn::PathArguments {
    fn conv_to_key_str(&self, f: &mut String) -> ChassisResult<()> {
        match &self {
            syn::PathArguments::None => Ok(()),
            syn::PathArguments::AngleBracketed(angle) => angle.conv_to_key_str(f),
            syn::PathArguments::Parenthesized(_) => Err(ChassisError::IllegalInput(
                "functions are not supported".into(),
                self.span(),
            )),
        }
    }
}

impl ToKeyStr for syn::AngleBracketedGenericArguments {
    fn conv_to_key_str(&self, f: &mut String) -> ChassisResult<()> {
        if self.colon2_token.is_some() {
            f.write_str("::")?;
        }
        f.write_str("<")?;
        dump_punctuated(",", &self.args, f)?;
        f.write_str(">").map_err(|err| err.into())
    }
}

impl ToKeyStr for syn::GenericArgument {
    fn conv_to_key_str(&self, f: &mut String) -> ChassisResult<()> {
        match self {
            GenericArgument::Type(ty) => ty.conv_to_key_str(f),
            GenericArgument::Binding(binding) => binding.conv_to_key_str(f),

            GenericArgument::Lifetime(_) => Err(ChassisError::IllegalInput(
                "lifetime specifiers are not supported".into(),
                self.span(),
            )),
            GenericArgument::Constraint(_) => Err(ChassisError::IllegalInput(
                "constraints are not supported".into(),
                self.span(),
            )),
            GenericArgument::Const(_) => Err(ChassisError::IllegalInput(
                "expressions in type are not allowed".into(),
                self.span(),
            )),
        }
    }
}

impl ToKeyStr for syn::Binding {
    fn conv_to_key_str(&self, f: &mut String) -> ChassisResult<()> {
        self.ident.conv_to_key_str(f)?;
        f.write_str("=")?;
        self.ty.conv_to_key_str(f)
    }
}

impl ToKeyStr for syn::Ident {
    fn conv_to_key_str(&self, f: &mut String) -> ChassisResult<()> {
        f.write_fmt(format_args!("{}", self))
            .map_err(|err| err.into())
    }
}

impl ToKeyStr for syn::TypePtr {
    fn conv_to_key_str(&self, f: &mut String) -> ChassisResult<()> {
        if self.mutability.is_some() {
            f.write_str("*mut ")?;
        } else {
            f.write_str("*const ")?;
        }
        self.elem.conv_to_key_str(f)
    }
}

impl ToKeyStr for syn::TypeReference {
    fn conv_to_key_str(&self, f: &mut String) -> ChassisResult<()> {
        if self.lifetime.is_some() {
            return Err(ChassisError::IllegalInput(
                "lifetime specifiers are not supported".into(),
                self.span(),
            ));
        }

        if self.mutability.is_some() {
            f.write_str("&mut ")?;
        } else {
            f.write_str("&")?;
        }
        self.elem.conv_to_key_str(f)
    }
}

impl ToKeyStr for syn::TypeTraitObject {
    fn conv_to_key_str(&self, f: &mut String) -> ChassisResult<()> {
        f.write_str("dyn ")?;
        dump_punctuated("+", &self.bounds, f)
    }
}

impl ToKeyStr for syn::TypeParamBound {
    fn conv_to_key_str(&self, f: &mut String) -> ChassisResult<()> {
        match self {
            TypeParamBound::Trait(trait_) => trait_.conv_to_key_str(f),
            TypeParamBound::Lifetime(_) => Err(ChassisError::IllegalInput(
                "lifetime specifiers are not supported".into(),
                self.span(),
            )),
        }
    }
}

impl ToKeyStr for syn::TraitBound {
    fn conv_to_key_str(&self, f: &mut String) -> ChassisResult<()> {
        match &self.modifier {
            TraitBoundModifier::None => {}
            TraitBoundModifier::Maybe(_) => f.write_str("?")?,
        }

        if self.lifetimes.is_some() {
            return Err(ChassisError::IllegalInput(
                "lifetime specifiers are not supported".into(),
                self.span(),
            ));
        }

        self.path.conv_to_key_str(f)
    }
}

impl ToKeyStr for syn::TypeTuple {
    fn conv_to_key_str(&self, f: &mut String) -> ChassisResult<()> {
        f.write_str("(")?;
        dump_punctuated(",", &self.elems, f)?;
        f.write_str(")").map_err(|err| err.into())
    }
}

impl ToKeyStr for syn::TypeImplTrait {
    fn conv_to_key_str(&self, f: &mut String) -> ChassisResult<()> {
        f.write_str("impl ")?;
        dump_punctuated("+", &self.bounds, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_type() {
        let ty: syn::Type = syn::parse2(quote! { String }).unwrap();
        assert_eq!("String", ty.to_key_str().unwrap());
    }

    #[test]
    fn check_paren() {
        let ty: syn::Type = syn::parse2(quote! { ((String)) }).unwrap();
        assert_eq!("String", ty.to_key_str().unwrap());
    }

    #[test]
    fn check_path() {
        let ty: syn::Type = syn::parse2(quote! { a::b }).unwrap();
        assert_eq!("a::b", ty.to_key_str().unwrap());
    }

    #[test]
    fn check_full_path() {
        let ty: syn::Type = syn::parse2(quote! { crate::my::Type }).unwrap();
        assert_eq!("crate::my::Type", ty.to_key_str().unwrap());
    }

    #[test]
    fn check_tuple() {
        let ty: syn::Type = syn::parse2(quote! { ( String, u32 ) }).unwrap();
        assert_eq!("(String,u32)", ty.to_key_str().unwrap());
    }

    #[test]
    fn check_generic() {
        let ty: syn::Type = syn::parse2(quote! { Arc<String> }).unwrap();
        assert_eq!("Arc<String>", ty.to_key_str().unwrap());
    }

    #[test]
    fn check_ref() {
        let ty: syn::Type = syn::parse2(quote! { &String }).unwrap();
        assert_eq!("&String", ty.to_key_str().unwrap());
    }

    #[test]
    fn check_mut_ref() {
        let ty: syn::Type = syn::parse2(quote! { &mut String }).unwrap();
        assert_eq!("&mut String", ty.to_key_str().unwrap());
    }

    #[test]
    fn check_dyn_bound() {
        let ty: syn::Type = syn::parse2(quote! { dyn Trait1 + Trait2 }).unwrap();
        assert_eq!("dyn Trait1+Trait2", ty.to_key_str().unwrap());
    }

    #[test]
    fn check_bound() {
        let ty: syn::Type = syn::parse2(quote! { Trait1 + Trait2 }).unwrap();
        assert_eq!("dyn Trait1+Trait2", ty.to_key_str().unwrap());
    }

    #[test]
    fn check_impl_bound() {
        let ty: syn::Type = syn::parse2(quote! { impl Trait1 + Trait2 }).unwrap();
        assert_eq!("impl Trait1+Trait2", ty.to_key_str().unwrap());
    }
}
