#![allow(dead_code, unused_variables, unused_imports, unused_imports, unused_mut
, non_camel_case_types)]
mod constructor;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned, TokenStreamExt, ToTokens};
use quote::__private::ext::RepToTokensExt;
use syn::{Ident, Data, Fields, Field, FieldsNamed};
use syn::spanned::Spanned;

/// This generate a custom model for each one struct that becomes to a model
/// generate the trait Model_{struct name} and create the constructor and relations for it
pub struct ModelGenerator<'a> {
    struct_name: Ident,
    struct_data: &'a Data,
    struct_fields: &'a FieldsNamed,
}

impl<'a> ModelGenerator<'a> {
    pub fn new(struct_data: &'a Data, struct_name: Ident) -> Self {
        Self {
            struct_name,
            struct_data,
            struct_fields: Self::extract_struct_fields(struct_data),
        }
    }

    /// Extract Named Fields from the Struct Data
    fn extract_struct_fields(struct_data: &Data) -> &FieldsNamed {
        if let Data::Struct(the_data) = struct_data {
            match &the_data.fields {
                Fields::Named(struct_members) => {
                    return struct_members;
                }
                _ => unimplemented!()
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

    pub fn create_impl(&self) -> TokenStream {
        let model_name = self.get_model_name();
        let struct_name = &self.struct_name;
        let constructor = self.create_impl_constructor();

        quote! {

            impl #model_name for #struct_name {
                #constructor
            }
        }
    }

    pub fn get_model_name(&self) -> Ident {
        format_ident!("Model{}",self.struct_name)
    }

}

