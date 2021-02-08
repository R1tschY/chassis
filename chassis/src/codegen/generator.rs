use proc_macro2::TokenStream as TokenStream2;

pub struct ComponentField {
    name: syn::Ident,
    ty: syn::Type,
    init: TokenStream2,
}

pub struct ComponentBuilder {
    fields: Vec<ComponentField>,
}

impl ComponentBuilder {
    pub fn new() -> Self {
        Self { fields: vec![] }
    }

    pub fn field(&mut self, name: syn::Ident, ty: syn::Type, init: TokenStream2) -> &mut Self {
        self.fields.push(ComponentField { name, ty, init });
        self
    }

    pub fn build(&mut self, impl_name: &syn::Ident) -> TokenStream2 {
        let singleton_defs: Vec<TokenStream2> = self
            .fields
            .iter()
            .map(|field| {
                let name = &field.name;
                let ty = &field.ty;
                quote! {
                    #name: #ty
                }
            })
            .collect();
        let singleton_inits: Vec<TokenStream2> = self
            .fields
            .iter()
            .map(|field| {
                let name = &field.name;
                let init = &field.init;
                let ty = &field.ty;
                quote! {
                    let #name: #ty = #init;
                }
            })
            .collect();

        let singleton_names: Vec<&syn::Ident> =
            self.fields.iter().map(|field| &field.name).collect();

        quote! {
            pub struct #impl_name {
                #(#singleton_defs),*
            }

            impl #impl_name {
                pub fn new() -> Self {
                    #(#singleton_inits)*
                    Self {
                        #(#singleton_names),*
                    }
                }
            }
        }
    }
}
