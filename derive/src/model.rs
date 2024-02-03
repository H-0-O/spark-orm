use proc_macro::TokenStream;

use quote::{quote, ToTokens};
use syn::{ItemStruct, Type};
use syn::GenericParam;
use crate::ModelArgs;

use crate::utility::GeneratorResult;

pub fn generate(__struct: &ItemStruct, model_args: ModelArgs) -> GeneratorResult<TokenStream> {
    let ident = &__struct.ident;
    let visibility = &__struct.vis;
    let (impl_generics, _ty_generics, where_clause) = __struct.generics.split_for_impl();
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
    let collection_name = model_args.coll_name;
    Ok(quote!(
        #[derive(serde::Serialize , serde::Deserialize , Debug , Default , rm_orm::TModel)]
        #[coll_name=#collection_name]
        #visibility struct #ident #impl_generics #where_clause {
           #filed_expand
           #other_field
        }
    )
        .into())
}

fn generate_other_filed(__struct: &ItemStruct) -> proc_macro2::TokenStream {
    let gen = &__struct.generics.params;

    let mut gens = vec![];
    gen.iter().for_each(|gp| {
        if let GenericParam::Type(tp) = gp {
            gens.push(tp.clone());
        }
    });

    let is_generic = |x: &Type| -> bool{
        gens.iter().any(|f| {
            f.ident.to_string() == x.to_token_stream().to_string()
        })
    };

    //TODO clean this piece of code
    let mut other_field = quote!();
    __struct.fields.iter().for_each(|field| {
        let ident = field.ident.as_ref().unwrap();
        let filed_type = &field.ty;
        let is_generic = is_generic(filed_type);
        let mut attrs = quote!();
        // collect all developer attributes
        field.attrs.iter().for_each(|attr| {
            attrs = quote!(
                #attrs

                #attr
            );
        });
        let generic_att = if is_generic {
            let deserialize_string = format!("{} : DeserializeOwned", filed_type.to_token_stream().to_string());
            quote!(  #[serde(bound(deserialize = #deserialize_string))] )
        } else { quote!() };
        other_field = quote!(
            #other_field

            #attrs
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

