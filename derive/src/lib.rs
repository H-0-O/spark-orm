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

use model::ModelGenerator;
use proc_macro2::Ident;
use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::spanned::Spanned;
use syn::{parse_macro_input, DeriveInput, Expr, Field, Fields, Member};

mod model;
mod utility;

#[proc_macro_derive(Model, attributes(model))]
pub fn model(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let mut model = ModelGenerator::new(&input);
    model.create_indexes();

    //TODO search that we need the trait for this or not

    // let the_trait = model.create_trait();

    let the_impl = model.create_impl();

    let expanded = quote! {
        #the_impl
    };
    // println!("the result is {:?} " , expanded.to_string());
    TokenStream::from(expanded)
}
