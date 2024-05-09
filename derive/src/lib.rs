extern crate darling;
extern crate once_cell;
extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

use darling::{FromMeta};
use syn::{ItemStruct, parse_macro_input};


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
#[derive(FromMeta, Debug)]
struct ModelArgs {
    coll_name: String,
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Model(args: TokenStream, item: TokenStream) -> TokenStream {
    let __struct = parse_macro_input!(item as ItemStruct);
    let model_args = parse_nested_meta!(ModelArgs , args);
    match model::generate(&__struct, &model_args) {
        Ok(expanded) => expanded,
        Err(err) => err.write_errors().into(),
    }
}
