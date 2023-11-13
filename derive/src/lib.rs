#![allow(dead_code)]
extern crate proc_macro;
extern crate quote;
extern crate syn;
extern crate proc_macro2;

use proc_macro::TokenStream;
use proc_macro2::Ident;

use quote::{format_ident, quote, quote_spanned};
use syn::{DeriveInput, Fields, parse_macro_input};
use syn::spanned::Spanned;

#[proc_macro_derive(Model)]
pub fn model(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    // let the_new = impl_model(&input.data , &name);
    println!("hello in compile time");
    let trait_name = format_ident!("Model_{}" , name);
    let expanded = quote! {
        pub trait #trait_name{
            fn print_name(l_name: &str);
        }

        impl #trait_name for #name{

            fn print_name(l_name: &str) {
                println!(" The struct is {:?} " , l_name);
            }
        }
    };
    println!(" the exp code {:?} " , expanded.to_string());

    TokenStream::from(expanded)
}


fn impl_model(data: &syn::Data , struct_name: &Ident) -> proc_macro2::TokenStream {
    println!("in impl model ");
    if let syn::Data::Struct(data) = data {
        match data.fields {
            Fields::Named(ref fields) => {
                let field_params = fields.named.iter().map(|fe| {
                    let name = &fe.ident;
                    let ty = &fe.ty;
                    quote_spanned! {name.span()=>
                        #name: #ty
                    }
                });
                let field_params_clone = fields.named.iter().map(|fe| {
                    let name = &fe.ident;
                    quote_spanned! {name.span()=>
                        #name: #name
                    }
                });
                let rs = quote! {
                     fn new( #(#field_params ),*) -> Self::TargetStruct {
                        Self {
                            #(#field_params_clone),*
                        }
                    }
                };

                println!("the re {:?} " , &rs.to_string());
                rs
            }
            _ => {
                quote! {
                      fn new() -> Self::TargetStruct {
                        Self {}
                    }
                }
            }
        }
    } else{
        quote! {
              fn new() -> Self::TargetStruct {
                        Self {}
                    }
        }
    }
}