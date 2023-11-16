#![allow(dead_code,
unused_variables,
unused_imports, unused_imports, unused_mut,
non_camel_case_types
)]

extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

use proc_macro2::Ident;
use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::{DeriveInput, Fields, parse_macro_input};
use syn::spanned::Spanned;

use model::ModelGenerator;

mod model;

#[proc_macro_derive(Model, attributes(trait_only))]
pub fn model(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let model = ModelGenerator::new(&input.data, name.clone());

    let trait_only = input.attrs.iter().any(|attr| {
        attr.path().is_ident("trait_only")
    });

    let the_trait = model.create_trait();
    let the_impl = if !trait_only {
        model.create_impl()
    } else {
        quote!()
    };
    let model_name = model.get_model_name();

    let expanded = quote! {
            #the_trait

            #the_impl
    };
    TokenStream::from(expanded)
}
