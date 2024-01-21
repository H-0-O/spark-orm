use proc_macro::TokenStream;
use quote::quote;
use syn::{Error, ItemStruct};
use crate::utility::GeneratorResult;

pub fn generate(__struct: &ItemStruct) -> GeneratorResult<TokenStream> {
    let ident = &__struct.ident;
    let (impl_generics , ty_generics , where_clause) = __struct.generics.split_for_impl();
    let have_id = __struct.fields.iter().any(|x|{
        x.ident.as_ref().unwrap().eq("_id")
    });
    if have_id {
        return Err(
            Error::new_spanned(ident , "_id can not be set manually ").into()
        )
    }

    println!("Have ID is {:?} " , have_id);
    Ok(quote!().into())
}