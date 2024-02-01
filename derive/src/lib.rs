#![allow(
dead_code,
unused_variables,
unused_imports,
unused_imports,
unused_mut,
non_camel_case_types
)]
extern crate darling;
extern crate once_cell;
extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use std::collections::HashSet;
use std::io::read_to_string;
use darling::{FromDeriveInput, FromMeta};

use proc_macro2::Ident;
use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::spanned::Spanned;
use syn::token::Struct;
use syn::{parse_macro_input, DeriveInput, Expr, Field, Fields, ItemStruct, Member, Path};

use crate::_model::__struct;

mod _model;
mod model;
mod utility;

/// Procedural macro to derive the `Model` trait for a struct.
///
/// This macro processes the input struct marked with the #[_model] attribute and generates
/// an implementation of the `Model` trait. The trait includes a constructor method and
/// an index registration method. The constructor initializes the struct and registers
/// its indexes in a MongoDB collection. The #[_model] attribute is used to annotate the
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
        #the_trait
        #the_impl
    };
    // println!("the expanded {:?} " , expanded.to_string() );
    // TokenStream::from(expanded)
    expanded.into()
}

#[proc_macro_attribute]
pub fn create_model(attr: TokenStream, item: TokenStream) -> TokenStream {
    let __struct = parse_macro_input!(item as ItemStruct);
    let name = &__struct.ident;
    match model::generate(&__struct) {
        Ok(expanded) => expanded,
        Err(err) => err.write_errors().into(),
    }
    // let token = quote!(
    //     struct #name{
    //         _id: String
    //     }
    // );
    // TokenStream::from(token)
}
