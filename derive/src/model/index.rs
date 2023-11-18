use crate::model::{ModelGenerator, MODEL_TRAIT_NAME};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{Attribute, Field, Meta};

impl<'a> ModelGenerator<'a> {
    pub fn create_indexes(&mut self) {
        self.struct_fields.named.iter().for_each(|field| {
            field.attrs.iter().for_each(|attr| {
                Self::is_attr_model(attr);
                Self::is_outer_field(attr);
                let field = field.ident.as_ref().unwrap();
                self.identify_attrs(attr, field.to_string())
            });
        });
    }
    fn is_outer_field(attr: &Attribute) {
        match attr.style {
            syn::AttrStyle::Outer => {}
            _ => unimplemented!(),
        }
    }
    fn is_attr_model(attr: &Attribute) {
        if !attr.meta.path().is_ident(&MODEL_TRAIT_NAME) {
            unimplemented!()
        }
    }
    fn identify_attrs(&mut self, attr: &Attribute, field_name: String) {
        if let Meta::List(listed_attr) = &attr.meta {
            let cloned_tokens = listed_attr.tokens.clone();
            cloned_tokens.into_iter().for_each(|token| {
                if token.to_string() == "unique" {
                    self.fields_attr.uniques.insert(field_name.clone());
                } else if token.to_string() == "index" {
                    self.fields_attr.indexes.insert(field_name.clone());
                }
            });
        }
    }
    pub fn create_index_creator_function(&self) -> TokenStream {
        let mut body_list = quote!();
        self.fields_attr
            .uniques
            .iter()
            .for_each(|field| body_list = quote! {#body_list #field,});
        let body_count = self.fields_attr.uniques.iter().count();
        let span = body_list.span();
        let uniques = vec![2];

        quote_spanned! {span=>
            pub async fn create_indexes(&self , db: &mongodb::Database , collection_name: &str){
                let uniques : Vec<&'static str>  = vec![#body_list];
                static INITIAL: once_cell::sync::OnceCell<()> = once_cell::sync::OnceCell::new();
                INITIAL.get_or_init( ||{
                    uniques.iter().for_each(|field|{
                        let name = format!("__{}__" , field).to_owned();
                        let clonedd_field = field.clone();
                        tokio::spawn(async move{
                            let index_model = rspark::utilities::create_index_on_model(clonedd_field, &name , true);
                            let _ = db.collection::<Self>(collection_name).create_index(index_model , None).await;
                        });
                    });
                });
            }
        }
    }
}
