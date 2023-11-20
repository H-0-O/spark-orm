use crate::model::MODEL_TRAIT_NAME;
use proc_macro2::{Span, TokenStream, TokenTree};
use quote::{format_ident, quote, quote_spanned};
use std::collections::HashSet;
use syn::spanned::Spanned;
use syn::{Attribute, Field, FieldsNamed, Meta};
///Manages the index-related attributes for a model.

/// In our database setup, we establish index fields for all attributes marked with indexing specifications,
/// such as unique or index constraints.
/// This work is performed only once during the initial instantiation of our model when creating an instance for the first time,
/// ensuring efficiency and optimal performance.

/// Manages the index-related attributes for a model.
///
/// This struct holds attributes for unique and indexed fields. The attributes are processed during
/// the initial instantiation of the model to establish index fields based on specified constraints
/// such as unique or index.
pub struct IndexManager {
    uniques: HashSet<String>,
    indexes: HashSet<String>,
}

impl IndexManager {
    /// Creates a new instance of the `IndexManager`.
    pub fn new() -> Self {
        Self {
            uniques: HashSet::new(),
            indexes: HashSet::new(),
        }
    }

    /// Registers unique and indexed fields based on the attributes of each field within a struct.
    ///
    /// This method identifies fields marked with the model trait attribute, extracts their names,
    /// and determines whether they are used in an outer attribute style. Unique and index fields
    /// are then recorded in the corresponding sets within the `IndexManager` instance.
    ///
    /// # Arguments
    ///
    /// - `fields_named`: A reference to the `FieldsNamed` struct containing the named fields of the target struct.
    ///
    /// # Returns
    ///
    /// A `TokenStream` representing the body of the function responsible for registering indexes.
    pub fn register_indexes(&mut self, fields_named: &FieldsNamed) -> TokenStream {
        fields_named.named.iter().for_each(|field: &Field| {
            field.attrs.iter().for_each(|attribute: &Attribute| {
                if Self::is_model_attribute(attribute) {
                    Self::check_outer_style(attribute);
                    let field_name = field.ident.as_ref().unwrap().to_string();
                    self.process_attribute(attribute, field_name);
                }
            });
        });
        self.generate_index_registration_body()
    }

    /// Generates the body of the index registration function.
    ///
    /// This function creates vectors of unique and indexed field names based on the information stored
    /// in the `IndexManager` instance. The function is designed to be asynchronous to allow for concurrent
    /// index creation. The registration process is performed only once using the `once_cell` crate to
    /// ensure efficiency and optimal performance.
    ///
    /// # Returns
    ///
    /// A `TokenStream` representing the body of the index registration function.
    ///
    /// # Notes
    ///
    /// The TODO section should be uncommented and completed with the necessary logic to register indexes.
    fn generate_index_registration_body(&self) -> TokenStream {
        let unique_list = Self::convert_attributes_to_list_tokens(&self.uniques);
        let index_list = Self::convert_attributes_to_list_tokens(&self.indexes);
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

    /// Processes the attributes of a field, focusing on attributes marked with the model trait.
    ///
    /// This method processes the attribute's tokens to identify unique and indexed fields,
    /// updating the corresponding sets within the `IndexManager` instance.
    ///
    /// # Arguments
    ///
    /// - `attr`: A reference to the attribute being processed.
    /// - `field_name`: The name of the field to which the attribute belongs.
    ///
    /// # Notes
    ///
    /// This method is designed to handle attributes with key-value pairs in a list style. It
    /// specifically looks for tokens representing "unique" and "index" to determine the type of the field.
    /// The identified fields are then added to the appropriate sets for further processing.
    fn process_attribute(&mut self, attr: &Attribute, field_name: String) {
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

    /// Converts a set of attribute names into a `TokenStream` that represents a list of names,
    /// suitable for use in code generation. Each attribute name is separated by commas and
    /// included in the resulting `TokenStream`.
    ///
    /// # Arguments
    ///
    /// - `attributes`: A reference to a `HashSet` containing attribute names to be converted.
    ///
    /// # Returns
    ///
    /// A `TokenStream` representing a list of attribute names suitable for code generation.
    fn convert_attributes_to_list_tokens(attributes: &HashSet<String>) -> TokenStream {
        let span = Span::mixed_site();
        let mut list = quote!();
        attributes.iter().for_each(|attr_name| {
            list = quote_spanned! {span=>
                #list #attr_name ,
            };
        });
        list
    }

    /// Checks whether the given attribute is associated with the model trait.
    ///
    /// # Arguments
    ///
    /// - `attr`: A reference to the attribute being checked.
    ///
    /// # Returns
    ///
    /// `true` if the attribute is associated with the model trait; otherwise, `false`.
    ///
    fn is_model_attribute(attr: &Attribute) -> bool {
        attr.meta.path().is_ident(&MODEL_TRAIT_NAME)
    }
    /// Checks whether the given attribute is applied in the outer attribute style.
    ///
    /// # Arguments
    ///
    /// - `attr`: A reference to the attribute being checked.
    ///
    /// # Notes
    ///
    /// The outer attribute style is where attributes are placed before the item they are modifying.
    /// If the attribute is not in the outer style, the function panics with an "unimplemented!" call.
    /// This implies that the attribute must be in the outer style for the current implementation.
    ///
    fn check_outer_style(attr: &Attribute) {
        match attr.style {
            syn::AttrStyle::Outer => {}
            _ => unimplemented!("The model macro must be Outer like this ( #[...]  )"),
        }
    }
}
