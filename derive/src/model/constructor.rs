use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::spanned::Spanned;
use syn::{Field, FieldsNamed};

use crate::model::{__struct, PROXY_MODEL_STRUCT_NAME};

impl __struct {
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
    #[doc(hidden)]
    /// it assume be like this
    ///  inner_instance = Self{           
    ///                      id: None,
    ///                    inner: Box::new( Book {  name ,  author , .... } ),
    ///                     db: dbInstance,
    ///                     collection_name: #collection_name
    /// }
    pub(in crate::model) fn generate_constructor(&self, fields_named: &FieldsNamed) -> TokenStream {
        let st_model = self.generate_base_model_instance();
        let base_model_name = self.get_proxy_model_ident();
        quote! {
                async fn new<'a>(db: &'a Database) -> #base_model_name<'a , Self> 
            {
                #st_model
            }
        }
    }
    fn generate_base_model_instance(&self) -> TokenStream {
        let name = self.get_proxy_model_ident();
        let collection_name = self.get_collection_name();
        quote! {
             #name::new(db , #collection_name )
        }
    }

    fn get_proxy_model_ident(&self) -> Ident {
        format_ident!("{}", PROXY_MODEL_STRUCT_NAME)
    }
}
