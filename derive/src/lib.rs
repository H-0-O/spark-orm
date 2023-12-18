#![allow(
    dead_code,
    unused_variables,
    unused_imports,
    unused_imports,
    unused_mut,
    non_camel_case_types
)]

extern crate once_cell;
extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use std::collections::HashSet;

use proc_macro2::Ident;
use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::{DeriveInput, Expr, Field, Fields, Member, parse_macro_input};
use syn::spanned::Spanned;

use crate::model::__struct;

mod model;
mod utility;

/// Procedural macro to derive the `Model` trait for a struct.
///
/// This macro processes the input struct marked with the #[model] attribute and generates
/// an implementation of the `Model` trait. The trait includes a constructor method and
/// an index registration method. The constructor initializes the struct and registers
/// its indexes in a MongoDB collection. The #[model] attribute is used to annotate the
/// struct that should be treated as a model.
///
/// # Example
///
/// ```rust
/// #[derive(Model)]
/// struct Book {
///     #[model(unique)]
///     title: String,
///     #[model(index)]
///     subject: &'static str,
///     author: Option<String>
/// }
/// ```
///
/// # Attributes
///
/// - `#[model]`: Annotates the struct to indicate that it should be treated as a model.
///
/// # Returns
///
/// A `TokenStream` representing the expanded code with the generated implementation
/// of the `Model` trait for the input struct.
#[proc_macro_derive(Model, attributes(model, coll_name))]
pub fn model(input: TokenStream) -> TokenStream {
    // Parse the input into a DeriveInput struct
    let input = parse_macro_input!(input as DeriveInput);

    // Create a new instance of the __struct struct to process the input
    let model = __struct::new(input);
    let the_trait = model.generate_trait();
    // Generate the implementation of the Model trait
    let the_impl = model.generate_impl();
    // Create the expanded TokenStream containing the generated code
    let expanded = quote! {
        use rm_orm::{
                     model::base_model::BaseModel, model::crud::base_model_crud::BaseModelCrud,
                    model::crud::inner_crud::InnerCRUD
        };
        #the_trait
        #the_impl
    };
    // println!("the expanded {:?} " , expanded.to_string() );
    // TokenStream::from(expanded)
    expanded.into()
}
