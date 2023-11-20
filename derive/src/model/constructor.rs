use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{Field, FieldsNamed};

pub(in crate::model) fn __constructor(
    fields_named: &FieldsNamed,
    collection_name: &str,
) -> TokenStream {
    //TODO later collection_name must define near the model macro
    let parameters = __model_constructor_parameters(fields_named);
    let struct_instance = __model_instance_fields(fields_named);
    quote!(
        async fn new(#parameters db: &mongodb::Database) -> Result<Self , ()>{
            let instance = Self {
                #struct_instance
            };

            instance.register_indexes(db , #collection_name).await;
            Ok(instance)
        }
    )
}

/// generate the function parameters like
///  pub fn (var1 : String , var2 : i8)
fn __model_constructor_parameters(fields_named: &FieldsNamed) -> TokenStream {
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
/// insert the instance of struct like this
/// Self { var1 , var2 }  this method generate fields between brackets
fn __model_instance_fields(fields_named: &FieldsNamed) -> TokenStream {
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
