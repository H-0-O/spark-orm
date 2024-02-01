use crate::_model::__struct;
use crate::utility::GeneratorResult;
use proc_macro::TokenStream;
use std::any::Any;
use std::process::id;
use darling::usage::UsesTypeParams;
use quote::{quote, ToTokens};
use quote::__private::ext::RepToTokensExt;
use syn::{Error, ItemStruct, Type};
use syn::GenericParam;

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
    let mut gen = &__struct.generics.params;

    let mut gens = vec![];
    gen.iter().for_each(|gp|{
        if let GenericParam::Type(tp) = gp{
            gens.push(tp.clone());
        }
    });

    let is_generic = |x: &Type|->bool{
        gens.iter().any(|f|{
            f.ident.to_string() == x.to_token_stream().to_string()
        })
    };

    //TODO clean this piece of code
    let mut other_field = quote!();
    __struct.fields.iter().for_each(|field| {
        let ident = field.ident.as_ref().unwrap();
        let filed_type = &field.ty;
        let is_generic = is_generic(filed_type);
        println!("is a generic {:?} , {:?} " , ident.to_string() ,is_generic);
        let field_attr = quote!();
        println!("before foreach ");
        field.attrs.iter().for_each(|attr|{
           let atr_token = attr.into_token_stream();
            println!("attr token {:?} " , atr_token.to_string());
        });
        let generic_att= if is_generic{
            quote!(  #[serde(bound(deserialize = "T: DeserializeOwned"))] )
        }else{quote!()};
        other_field = quote!(
            #other_field

            #generic_att
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

