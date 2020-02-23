use syn::Ident;

pub trait IdentExt {
    fn prepend(&self, string: &str) -> Ident;
}

impl IdentExt for Ident {
    fn prepend(&self, string: &str) -> Ident {
        Ident::new(&format!("{}{}", string, self), self.span())
    }
}
