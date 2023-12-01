use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, ToTokens};
use syn::{Field, FieldsNamed};
use syn::spanned::Spanned;

/// Generates the constructor function for the custom model.
///
/// This function produces an asynchronous constructor for the struct annotated with the `model` trait.
/// It includes the necessary parameters and creates an instance of the struct, registering its indexes.
///
/// # Arguments
///
/// - `fields_named`: A reference to the named fields of the struct.
///
/// # Returns
///
/// A `TokenStream` representing the generated constructor function
pub(in crate::model) fn generate_constructor(
    fields_named: &FieldsNamed,
) -> TokenStream {
    let parameters = generate_constructor_parameters(fields_named);
    let struct_instance = generate_struct_instance(fields_named);
    quote!(
        async fn new(#parameters) -> Result<Self , ()>{
            let instance = Self {
                #struct_instance
            };
            Ok(instance)
        }
    )
}

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
/// A `TokenStream` representing the function parameters.
fn generate_constructor_parameters(fields_named: &FieldsNamed) -> TokenStream {
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
fn generate_struct_instance(fields_named: &FieldsNamed) -> TokenStream {
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
