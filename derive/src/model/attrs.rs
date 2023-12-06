use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::{Attribute, Meta};
use crate::model::__struct;

impl __struct{
    pub(crate) fn get_collection_name(&self) -> Option<TokenStream>{
        for attr in &self.0.attrs{
            if let Meta::NameValue(named_attr) = &attr.meta{
                if named_attr.path.is_ident("coll_name"){
                    return Some(named_attr.value.to_token_stream())
                }
            }
        }
        None
    }
}
