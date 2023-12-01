use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::{Attribute, Meta};

pub(crate) fn get_collection_name(attrs: &Vec<Attribute> ) -> Option<TokenStream>{
    for attr in attrs{
        if let Meta::NameValue(named_attr) = &attr.meta{
            if named_attr.path.is_ident("coll_name"){
                return Some(named_attr.value.to_token_stream())
            }
        }
    }
    None
}

pub  fn generate_get_collection_name(attrs: &Vec<Attribute> , model_name: &Ident) -> TokenStream{
    let mut name;
    if let Some(col_name) = get_collection_name(attrs){
        name = col_name.to_string();
    }else{
        name = model_name.to_string();
    }
    quote! {
        fn get_collection_name() -> String{
           #name.to_string()
        }   
    }
}