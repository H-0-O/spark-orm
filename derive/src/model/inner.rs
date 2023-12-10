use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{Field, FieldsNamed};
use syn::spanned::Spanned;
use crate::model::__struct;

pub(crate) trait filler {
    fn generate_fill_method(&self) ->TokenStream;

    /// Generates the function parameters for the constructor.
    ///
    /// This function creates a list of function parameters based on the named fields of the struct.
    ///
    /// # Arguments
    ///
    /// - `fields_named`: A reference to the named fields of the struct.
    ///
    /// # Returns
    ///
    /// A `TokenStream` representing the function parameters
    fn generate_filler_parameters(&self , fields_named: &FieldsNamed) ->TokenStream;

    /// Generates the instance fields for the constructor body.
    ///
    /// This function creates a list of fields for the struct instance creation.
    ///
    /// # Arguments
    ///
    /// - `fields_named`: A reference to the named fields of the struct.
    ///
    /// # Returns
    ///
    /// A `TokenStream` representing the struct instance fields.
    fn generate_struct_instance(&self , fields_named: &FieldsNamed) -> TokenStream;
}
impl filler for __struct {
    fn generate_fill_method (&self) -> TokenStream {
        let fields = Self::extract_struct_fields(&self.0.data);
        let params = self.generate_filler_parameters(fields);
        let struct_instance = self.generate_struct_instance(fields);
        quote!{
            fn fill(&self , #params){
                    let instance = Self{#struct_instance};
            }
        }
    }
    fn generate_filler_parameters(&self , fields_named: &FieldsNamed) -> TokenStream {
        let mut parameters = quote!();
        fields_named.named.iter().for_each(|field: &Field| {
            let span = field.span();
            let field_name = field.ident.as_ref().unwrap();
            let field_type = &field.ty;
            let param = quote_spanned! {span=>
                #field_name : #field_type,
            };
            parameters = quote! {
                #parameters
                #param
            }
        });
        parameters
    }
    fn generate_struct_instance(&self, fields_named: &FieldsNamed) -> TokenStream {
        let mut body_params = quote!();
        fields_named.named.iter().for_each(|field: &Field| {
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