use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, quote_spanned};
use syn::spanned::Spanned;
use syn::Field;

use crate::model::ModelGenerator;

impl<'a> ModelGenerator<'a> {
    /// Create the create constructor in trait
    pub(in crate::model) fn create_trait_constructor(&self) -> TokenStream {
        let parameters = self.create_constructor_parameters();
        quote! {
            fn new(#parameters) -> Self;
        }
    }

    pub(in crate::model) fn create_constructor_function(&self) -> TokenStream {
        let parameters = self.create_constructor_parameters();
        let struct_instance = self.create_struct_instance();
        let collection_name = &self.struct_name.to_string();
        //TODO later collection_name must define near the model macro
        quote!(
            async fn new(#parameters db: &mongodb::Database) -> Result<Self , ()>{
                let instance = Self {
                    #struct_instance
                };

                instance.create_indexes(db , #collection_name).await;
                Ok(instance)
            }
        )
    }

    /// Create Parameter name and type for each struct parameter
    fn create_constructor_parameters(&self) -> TokenStream {
        let mut parameters = quote!();
        self.struct_fields.named.iter().for_each(|field: &Field| {
            let span = field.span();
            let field_name = field.ident.as_ref().unwrap();
            let field_type = &field.ty;
            let param = quote_spanned! {span=>
                #field_name:#field_type,
            };
            parameters = quote! {
                #parameters
                #param
            }
        });
        parameters
    }

    fn create_struct_instance(&self) -> TokenStream {
        let mut body_params = quote!();
        self.struct_fields.named.iter().for_each(|field: &Field| {
            let span = field.span();
            let field_name = field.ident.as_ref().unwrap();
            let field = quote_spanned! {span=>
                #field_name ,
            };
            body_params = quote! {
                #body_params
                #field
            }
        });
        body_params
    }
}
