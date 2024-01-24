use crate::_model::__struct;
use crate::utility::GeneratorResult;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{Error, ItemStruct};

pub fn generate(__struct: &ItemStruct) -> GeneratorResult<TokenStream> {
    let ident = &__struct.ident;
    let visibility = &__struct.vis;
    let (impl_generics, ty_generics, where_clause) = __struct.generics.split_for_impl();
    let mut filed_expand = quote!();
    if !check_filed_exists(__struct, "_id") { // check _id exists or not
        filed_expand = quote!(
            #[serde(skip_serializing_if = "Option::is_none")]
            _id: Option<mongodb::bson::oid::ObjectId>,
        )
    }
    if !check_filed_exists(__struct, "created_at") {
        filed_expand = quote!(
            #filed_expand

            #[serde(skip_serializing_if = "Option::is_none")]
            created_at: Option<rm_orm::types::DateTime>,
        )
    }
    if !check_filed_exists(__struct, "updated_at") {
        filed_expand = quote!(
            #filed_expand

            updated_at: rm_orm::types::DateTime,
        )
    }
    if !check_filed_exists(__struct, "deleted_at") {
        filed_expand = quote!(
            #filed_expand

            #[serde(skip_serializing_if = "Option::is_none")]
            deleted_at: Option<rm_orm::types::DateTime>,
        )
    }
    let other_field = generate_other_filed(__struct);
    //TODO coll_name must be get from user
    //TODO generic types must be annotated with the seder( (deserialize_with = "T::deserialize") or (bound(deserialize = "T: DeserializeOwned" ) )
    //TODO all developer attribute must forwarded here
    Ok(quote!(
        #[derive(serde::Serialize , serde::Deserialize , Debug , Default , rm_orm::Model)]
        #[coll_name="Model"]
        #visibility struct #ident #impl_generics #where_clause {
           #filed_expand
           #other_field
        }
    )
    .into())
}

fn generate_other_filed(__struct: &ItemStruct) -> proc_macro2::TokenStream {
    let mut other_field = quote!();
    __struct.fields.iter().for_each(|x| {
        let ident = x.ident.as_ref().unwrap();
        let filed_type = &x.ty;
        other_field = quote!(
            #other_field
            #ident : #filed_type ,
        );
    });
    other_field
}

fn check_filed_exists(__struct: &ItemStruct, field_name: &str) -> bool {
    __struct
        .fields
        .iter()
        .any(|x| x.ident.as_ref().unwrap().eq(field_name))
}
