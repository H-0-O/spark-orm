extern crate darling;
extern crate once_cell;
extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

use darling::{FromAttributes, FromDeriveInput, FromMeta};
use quote::{quote, ToTokens};
use syn::{DeriveInput, ItemStruct, parse_macro_input};
use syn::spanned::Spanned;

use crate::_model::__Struct;

mod _model;
mod model;
mod utility;

macro_rules! parse_nested_meta {
    ($ty:ty, $args:expr) => {{
        let meta = match darling::ast::NestedMeta::parse_meta_list(proc_macro2::TokenStream::from(
            $args,
        )) {
            Ok(v) => v,
            Err(e) => {
                return TokenStream::from(darling::Error::from(e).write_errors());
            }
        };

        match <$ty>::from_list(&meta) {
            Ok(object_args) => object_args,
            Err(err) => return TokenStream::from(err.write_errors()),
        }
    }};
}



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
#[proc_macro_derive(TModel, attributes(model, coll_name))]
pub fn model(input: TokenStream) -> TokenStream {
    // Parse the input into a DeriveInput struct
    let input = parse_macro_input!(input as DeriveInput);

    // Create a new instance of the __struct struct to process the input
    let model = __Struct::new(input);
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

#[derive(FromMeta , Debug)]
struct ModelArgs{
    coll_name: String
}
#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Model(args: TokenStream, item: TokenStream) -> TokenStream {
    let __struct = parse_macro_input!(item as ItemStruct);
    let model_args = parse_nested_meta!(ModelArgs , args);
    let name = &__struct.ident;
    match model::generate(&__struct , model_args) {
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
