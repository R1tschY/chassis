use syn::Ident;

pub trait IdentExt {
    fn as_ident(&self) -> &Ident;

    fn prepend(&self, string: &str) -> Ident {
        Ident::new(
            &format!("{}{}", string, self.as_ident()),
            self.as_ident().span(),
        )
    }

    fn append(&self, string: &str) -> Ident {
        Ident::new(
            &format!("{}{}", self.as_ident(), string),
            self.as_ident().span(),
        )
    }
}

impl IdentExt for Ident {
    fn as_ident(&self) -> &Ident {
        self
    }
}
