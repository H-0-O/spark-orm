use crate::model::MODEL_TRAIT_NAME;
use proc_macro2::{Span, TokenStream, TokenTree};
use quote::{format_ident, quote, quote_spanned};
use std::collections::HashSet;
use syn::spanned::Spanned;
use syn::{Attribute, Field, FieldsNamed, Meta};

pub struct __indexes {
    uniques: HashSet<String>,
    indexes: HashSet<String>,
}

impl __indexes {
    pub fn new() -> Self {
        Self {
            uniques: HashSet::new(),
            indexes: HashSet::new(),
        }
    }
    pub fn __register_indexes(&mut self, fields_named: &FieldsNamed) -> TokenStream {
        fields_named.named.iter().for_each(|field: &Field| {
            field.attrs.iter().for_each(|attribute: &Attribute| {
                Self::is_attr_model(attribute);
                Self::is_outer_field(attribute);
                let field_name = field.ident.as_ref().unwrap().to_string();

                self.split_the_attribute(attribute, field_name);
            });
        });
        self.__body()
    }
    /// Generates the function that call the RSparks' create_index_on_model function
    /// add the field as index and unique and ...
    fn __body(&self) -> TokenStream {
        let unique_list = Self::attributes_into_list(&self.uniques);
        let index_list = Self::attributes_into_list(&self.indexes);
        let span = Span::mixed_site();
        quote_spanned! {span=>
            pub async fn register_indexes (&self , db: &mongodb::Database , collection_name: &str ){
                let uniques : Vec<&str> = vec![#unique_list];
                let indexes : Vec<&str> =  vec![#index_list];
                static REGISTER_INDEXES_ONCE : once_cell::sync::OnceCell<()> = once_cell::sync::OnceCell::new();
                REGISTER_INDEXES_ONCE.get_or_init(||{
                    println!("Hello in do once");
                              // uniques.iter().for_each(|field|{
                    //     let name = format!("__{}__" , field).to_owned();
                    //     let clonedd_field = field.clone();
                    //     tokio::spawn(async move{
                    //         let index_model = rspark::utilities::create_index_on_model(clonedd_field, &name , true);
                    //         let _ = db.collection::<Self>(collection_name).create_index(index_model , None).await;
                    //     });
                    // });
                });
            }
        }
    }
    fn split_the_attribute(&mut self, attr: &Attribute, field_name: String) {
        if let Meta::List(listed_attr) = &attr.meta {
            let tokens = listed_attr.tokens.clone();
            tokens.into_iter().for_each(|token: TokenTree| {
                if &token.to_string() == "unique" {
                    self.uniques.insert(field_name.clone());
                } else if token.to_string() == "index" {
                    self.indexes.insert(field_name.clone());
                }
            });
        }
    }

    fn attributes_into_list(attributes: &HashSet<String>) -> TokenStream {
        let span = Span::mixed_site();
        let mut list = quote!();
        attributes.iter().for_each(|attr_name| {
            list = quote_spanned! {span=>
                #list #attr_name ,
            };
        });
        list
    }

    fn is_attr_model(attr: &Attribute) {
        if !attr.meta.path().is_ident(&MODEL_TRAIT_NAME) {
            unimplemented!()
        }
    }
    fn is_outer_field(attr: &Attribute) {
        match attr.style {
            syn::AttrStyle::Outer => {}
            _ => unimplemented!(),
        }
    }
}
