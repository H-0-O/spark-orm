#![allow(
    dead_code,
    unused_variables,
    unused_imports,
    unused_imports,
    unused_mut,
    non_camel_case_types
)]

use std::collections::HashSet;

use once_cell::sync::OnceCell;
use proc_macro2::TokenStream;
use quote::__private::ext::RepToTokensExt;
use quote::{format_ident, quote, quote_spanned, ToTokens, TokenStreamExt};
use syn::spanned::Spanned;
use syn::{Attribute, Data, DeriveInput, Field, Fields, FieldsNamed, Generics, Ident};

use crate::model::index::__indexes;

mod constructor;
mod index;

const MODEL_TRAIT_NAME: &'static str = "model";

/// This generate a custom model for each one struct that becomes to a model
/// generate the trait Model{struct name} ex( ModelUser ) and create the constructor and relations for it
pub struct __struct(DeriveInput);
impl __struct {
    pub fn new(input: DeriveInput) -> Self {
        Self(input)
    }
    pub fn generate_impl(self) -> TokenStream {
        //TODO collection name must get from the developer and the ident must be default for it
        let collection_name = self.0.ident.to_string();
        let model_name = self.0.ident;
        let fields_name = Self::extract_struct_fields(&self.0.data);
        let constructor = constructor::__constructor(fields_name, &collection_name);
        let (impl_generics, type_generics, where_generics) = self.0.generics.split_for_impl();
        let index_register = __indexes::new().__register_indexes(fields_name);
        quote! {
           impl #impl_generics #model_name #type_generics #where_generics {
                #constructor
                #index_register
            }
        }
    }

    fn get_model_name(&self) -> String {
        format_ident!("Model{}", self.0.ident).to_string()
    }

    fn extract_struct_fields(data: &Data) -> &FieldsNamed {
        if let Data::Struct(the_data) = data {
            match &the_data.fields {
                Fields::Named(struct_members) => {
                    return struct_members;
                }
                _ => unimplemented!(),
            }
        }
        unimplemented!()
    }
}
