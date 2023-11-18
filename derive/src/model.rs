#![allow(
    dead_code,
    unused_variables,
    unused_imports,
    unused_imports,
    unused_mut,
    non_camel_case_types
)]

mod constructor;
mod index;

use once_cell::sync::OnceCell;
use proc_macro2::TokenStream;
use quote::__private::ext::RepToTokensExt;
use quote::{format_ident, quote, quote_spanned, ToTokens, TokenStreamExt};
use std::collections::HashSet;
use syn::spanned::Spanned;
use syn::{Attribute, Data, DeriveInput, Field, Fields, FieldsNamed, Generics, Ident};

const MODEL_TRAIT_NAME: &'static str = "model";

/// This generate a custom model for each one struct that becomes to a model
/// generate the trait Model{struct name} ex( ModelUser ) and create the constructor and relations for it
pub struct ModelGenerator<'a> {
    struct_name: &'a Ident,
    struct_data: &'a Data,
    struct_fields: &'a FieldsNamed,
    struct_attrs: &'a Vec<Attribute>,
    generics: &'a Generics,
    fields_attr: FieldsAttr,
}

struct FieldsAttr {
    uniques: HashSet<String>,
    indexes: HashSet<String>,
}

impl<'a> ModelGenerator<'a> {
    pub fn new(input: &DeriveInput) -> ModelGenerator {
        ModelGenerator {
            struct_name: &input.ident,
            struct_data: &input.data,
            generics: &input.generics,
            struct_attrs: &input.attrs,
            struct_fields: Self::extract_struct_fields(&input.data),
            fields_attr: FieldsAttr {
                uniques: HashSet::new(),
                indexes: HashSet::new(),
            },
        }
    }

    /// Extract Named Fields from the Struct Data
    fn extract_struct_fields(struct_data: &Data) -> &FieldsNamed {
        if let Data::Struct(the_data) = struct_data {
            match &the_data.fields {
                Fields::Named(struct_members) => {
                    return struct_members;
                }
                _ => unimplemented!(),
            }
        }
        unimplemented!()
    }

    pub fn create_trait(&self) -> TokenStream {
        let model_name = self.get_model_name();
        let name = &self.struct_name;
        let constructor = self.create_trait_constructor();
        quote! {
            trait #model_name{
                #constructor
            }
        }
    }

    /// Here Collect All the functions and methods of struct
    pub fn create_impl(&self) -> TokenStream {
        let model_name = self.get_model_name();
        let struct_name = self.struct_name;
        let constructor = self.create_constructor_function();
        let index_creator = self.create_index_creator_function();
        println!("the index creator {:?} " , index_creator.to_string());
        let (impl_generics, type_generics, where_generics) = self.generics.split_for_impl();
        quote! {
            impl #impl_generics #struct_name #type_generics #where_generics {
                #constructor
                #index_creator
            }
        }
    }

    pub fn get_model_name(&self) -> Ident {
        format_ident!("Model{}", self.struct_name)
    }
}
