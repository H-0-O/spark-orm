use std::fmt::Display;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, TokenStreamExt, ToTokens};
use quote::__private::ext::RepToTokensExt;
use quote::spanned::Spanned;
use syn::{Data, DeriveInput, Fields, FieldsNamed};


mod constructor;
mod attrs;
mod index;
const MODEL_ATTRIBUTE_NAME: &'static str = "model";
const INNER_CRUD_TRAIT_NAME: &'static str = "InnerCRUD";

const BASE_MODEL_STRUCT_NAME: &'static str = "BaseModel";
/// This generate a custom model for each one struct that becomes to a model
/// generate the trait Model{struct name} ex( ModelUser ) and create the constructor and relations for it
pub struct __struct(DeriveInput);
impl __struct {
    pub fn new(input: DeriveInput) -> Self {
        Self(input)
    }

    pub fn generate_trait(&self) -> TokenStream{
        let model_name = &self.0.ident;
        let trait_name = format_ident!("{}" ,INNER_CRUD_TRAIT_NAME);
        let (impl_generics, type_generics, where_generics) = self.0.generics.split_for_impl();
        quote!{
            impl #impl_generics #trait_name for  #model_name #type_generics #where_generics {}
        }
    }
    /// Generates the implementation code for the custom model.
    ///
    /// This method takes the struct annotated with the `model` trait and produces the necessary
    /// implementation code for the associated custom model. It includes the construction logic
    /// and index registration for the model's fields. The generated implementation is ready for use
    /// when deriving the custom model for the specified struct.
    ///
    /// # Returns
    ///
    /// A `TokenStream` representing the implementation code for the custom model.
    ///
    pub fn generate_impl(&self) -> TokenStream {
        let model_name = &self.0.ident;
        let fields_name = Self::extract_struct_fields(&self.0.data);
        let constructor = self.generate_constructor(fields_name);
        let (impl_generics, type_generics, where_generics) = self.0.generics.split_for_impl();
        // TODO adapt with new structure
        quote! {
           impl #impl_generics #model_name #type_generics #where_generics {
                #constructor
            }
        }
    }

    /// Extracts the named fields from the struct data.
    ///
    /// This method is responsible for extracting the named fields from the struct data
    /// and returning them for further processing. It is designed to handle structs
    /// annotated with the `model` trait, ensuring that only named structs are supported.
    ///
    /// # Arguments
    ///
    /// - `data`: A reference to the struct data.
    ///
    /// # Returns
    ///
    /// A reference to the named fields within the struct data.
    ///
    /// # Panics
    ///
    /// Panics if the data structure is not a named struct, as only named structs are currently supported.
    ///
    fn extract_struct_fields(data: &Data) -> &FieldsNamed {
        if let Data::Struct(the_data) = data {
            match &the_data.fields {
                Fields::Named(struct_members) => {
                    return struct_members;
                }
                _ => unimplemented!("I have no idea about UnNamed or Units fields "),
            }
        }
        todo!("the Enum and Union not supported yet ");
    }

    fn get_model_name(&self) -> String {
        format_ident!("Model{}", self.0.ident).to_string()
    }
}
