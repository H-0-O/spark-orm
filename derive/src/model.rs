use proc_macro::TokenStream;
use darling::FromMeta;

use quote::{quote, ToTokens};
use syn::{Attribute, Generics, ImplGenerics, ItemStruct, Path, Type, TypeGenerics};
use syn::GenericParam;
use crate::ModelArgs;
use crate::utility::GeneratorResult;

const PROXY_MODEL_STRUCT_PATH: &str = "spark_orm::model::Model";
const MODEL_TIMESTAMPS_TRAIT_PATH: &str = "spark_orm::model::util::ModelTimestamps";
const MODEL_OBSERVER_TRAIT_PATH: &str = "spark_orm::model::observer::Observer";

pub fn generate(__struct: &ItemStruct, model_args: &ModelArgs) -> GeneratorResult<TokenStream> {
    let ident = &__struct.ident;
    let visibility = &__struct.vis;
    let (impl_generics, _ty_generics, where_clause) = prepare_generics(&__struct.generics);



    // this section forward developers attrs to here
    let struct_attrs = extract_struct_attrs(__struct);



    // this generates the fields of struct that developer defined
    let other_field = regenerate_defined_filed(__struct);

    //model creator implement
    let model_creator = generate_model_creator_impl(__struct, model_args);

    //this generates From trait to convert to document
    let from_to_document_trait = generate_from_to_document_trait(__struct);

    //this generates Observer<T> trait if user does' fill the observer
    let observer_trait = generate_observer_trait(__struct , model_args);

    // this there lines first inspect that the user defined timestamp or not then create fields
    // for them and after that defines the update method for them
    let mut time_creator = vec![];
    let filed_expand = generate_time_stamps(__struct, &mut time_creator);
    let date_time_functions = generate_date_times_functions(__struct, time_creator);

    Ok(
        quote!(
            #struct_attrs
            #visibility struct #ident #impl_generics #where_clause {
               #filed_expand
               #other_field
            }

            #model_creator

            #from_to_document_trait

            #date_time_functions

            #observer_trait
        ).into()
    )
}

/// this function first checks that user defined create_at or ...
/// then generates them if it isn't exist
fn generate_time_stamps(__struct: &ItemStruct, time_creator: &mut Vec<&str>)
                        -> proc_macro2::TokenStream {

    let mut filed_expand = quote!();
    if !check_filed_exists(__struct, "_id") { // check _id exists or not
        filed_expand = quote!(
            #[serde(skip_serializing_if = "Option::is_none")]
            pub _id: Option<mongodb::bson::oid::ObjectId>,
        )
    }
    if !check_filed_exists(__struct, "created_at") {
        time_creator.push("created_at");
        filed_expand = quote!(
            #filed_expand

            #[serde(default = "Option::default")]
            #[serde(skip_serializing_if = "Option::is_none")]
            pub created_at: Option<mongodb::bson::DateTime>,
        )
    }
    if !check_filed_exists(__struct, "updated_at") {
        time_creator.push("updated_at");
        filed_expand = quote!(
            #filed_expand

            #[serde(default = "Option::default")]
            #[serde(skip_serializing_if = "Option::is_none")]
            pub updated_at: Option<mongodb::bson::DateTime>,
        )
    }
    if !check_filed_exists(__struct, "deleted_at") {
        time_creator.push("deleted_at");
        filed_expand = quote!(
            #filed_expand

            #[serde(default = "Option::default")]
            #[serde(skip_serializing_if = "Option::is_none")]
            pub deleted_at: Option<mongodb::bson::DateTime>,
        )
    }

    filed_expand
}

/// this function extracts user defined attributes for his struct
fn extract_struct_attrs(__struct: &ItemStruct) -> proc_macro2::TokenStream {
    let mut struct_attrs = quote!();
    __struct.attrs.iter().for_each(|attr| {
        struct_attrs = quote!(
            #struct_attrs

            #attr
        )
    });
    struct_attrs
}

/// this function regenerates the user fields , all fields that user defined regenerate with their
/// attrs
fn regenerate_defined_filed(__struct: &ItemStruct) -> proc_macro2::TokenStream {
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
        let vis = &field.vis;
        let filed_type = &field.ty;
        let is_generic = is_generic(filed_type);

        // this is for handling missing fields without Option enum

        let mut attrs =
            if !attr_exists(&field.attrs, "serde(default)") &&
                !attr_exists(&field.attrs, "no_default")
            {
                quote!(
                #[serde(default)]
            )
            } else {
                quote!()
            };

        // collect all developer attributes
        field.attrs.iter().for_each(|attr| {
            if !is_custom_attribute(attr) {
                attrs = quote!(
                    #attrs

                    #attr
                );
            }
        });
        let generic_att = if is_generic {
            let deserialize_string = format!("{} : spark_orm::DeserializeOwned", filed_type.to_token_stream().to_string());
            quote!(  #[serde(bound(deserialize = #deserialize_string))] )
            // quote!(    #[serde(bound(deserialize = "T : serde::de::DeserializeOwned"))])
        } else { quote!() };
        other_field = quote!(
            #other_field

            #attrs
            #generic_att
            #vis #ident : #filed_type ,
        );
    });

    other_field
}


/// this function generates From trait for document and Model
fn generate_from_to_document_trait(__struct: &ItemStruct) -> proc_macro2::TokenStream {
    let model_name = &__struct.ident;
    let (impl_generics, ty_generics, where_clause) = prepare_generics(&__struct.generics);
    quote!(
        impl #impl_generics From<#model_name #ty_generics> for mongodb::bson::Document #where_clause {
             fn from(value: #model_name #ty_generics) -> Self {
                mongodb::bson::to_document(&value).unwrap()
            }
        }

        impl #impl_generics From<&#model_name #ty_generics> for mongodb::bson::Document #where_clause{
             fn from(value: &#model_name #ty_generics) -> Self {
                mongodb::bson::to_document(&value).unwrap()
            }
        }
    )
}

/// this function is just simple check that field exists on user defined struct or not
fn check_filed_exists(__struct: &ItemStruct, field_name: &str) -> bool {
    __struct
        .fields
        .iter()
        .any(|x| x.ident.as_ref().unwrap().eq(field_name))
}


/// this function generates now_model function to create  new instance of Model with user defined struct as inner
fn generate_model_creator_impl(__struct: &ItemStruct, model_args: &ModelArgs) -> proc_macro2::TokenStream {
    let model_name = &__struct.ident;
    let model = Path::from_string(PROXY_MODEL_STRUCT_PATH).unwrap();
    let coll_name = &model_args.coll_name;
    let register_attributes_function = generate_register_attribute_function(__struct);
    let (impl_generics, type_generics, where_generics) = prepare_generics(&__struct.generics);
    quote! {
           impl #impl_generics #model_name #type_generics
            #where_generics

            {
                pub fn new_model<'a>(db: Option<& std::sync::Arc<mongodb::Database>>) -> #model<'a , Self>{
                    let model = #model::<Self>::new(db , #coll_name);
                    Self::register_attributes(&model);
                    model
                }

                #register_attributes_function
            }
    }
}


/// this function generates ModelTimestamp traits to update document times when needed
fn generate_date_times_functions(__struct: &ItemStruct, exists_fields: Vec<&str>) -> proc_macro2::TokenStream {
    let model_name = &__struct.ident;
    let (impl_generics, type_generics, where_generics) = prepare_generics(&__struct.generics);
    let tr = Path::from_string(MODEL_TIMESTAMPS_TRAIT_PATH).unwrap();
    let mut qu = quote!();

    //TODO the ModelTimestamps must split into 3 trait (CreatedAt , UpdatedAt , DeletedAt)
    // because now user cant change them for self
    if exists_fields.iter().any(|x| *x == "created_at") {
        qu = quote! {
                fn created_at(&mut self){
                    self.created_at = Some(mongodb::bson::DateTime::now());
                }   
        };
    }

    if exists_fields.iter().any(|x| *x == "deleted_at") {
        qu = quote! {
              #qu
            
                fn updated_at(&mut self){
                    self.updated_at = Some(mongodb::bson::DateTime::now());
                }
        };
    }
    if exists_fields.iter().any(|x| *x == "deleted_at") {
        qu = quote! {
            #qu
            fn deleted_at(&mut self){
                self.deleted_at = Some(mongodb::bson::DateTime::now());
            }
        };
    }
    quote!(
        impl #impl_generics #tr for #model_name #type_generics #where_generics {   
            #qu  
        }
    )
}

/// attr_to_compare must be without # and [] , like serde(default)
fn attr_exists(attrs: &[Attribute], attr_to_compare: &str) -> bool {
    let mut has_it = false;
    attrs.iter().for_each(|attr| {
        if attr.meta.to_token_stream().to_string() == attr_to_compare {
            has_it = true;
        }
    });
    has_it
}


/// this function generate a function to call register attributes in database
fn generate_register_attribute_function(__struct: &ItemStruct) -> proc_macro2::TokenStream {
    let fields = &__struct.fields;
    let mut indexes = quote!();
    let model = Path::from_string(PROXY_MODEL_STRUCT_PATH).unwrap();

    fields.iter().for_each(|field| {
        if attr_exists(&field.attrs, "index") {
            let ident = field.ident.to_token_stream().to_string();
            indexes = quote!(
                    #indexes
                   
                    #ident, 
            );
        }
    });

    // println!("the indexes {:?}" , indexes.to_string());
    quote!(
        pub fn register_attributes<'a>(model: &#model<'a , Self>){
            let indexes = vec![#indexes];
            static registerer: std::sync::Once = std::sync::Once::new();
            //TODO this must be fix
            
            registerer.call_once( ||{
                model.register_attributes(indexes);
            });
        }
    )
}

/// this function first checks that user wants to use observer or not
/// if user wants we don't generate trait unless we generate just trait with its default functions
fn generate_observer_trait(__struct: &ItemStruct , args: &ModelArgs) -> proc_macro2::TokenStream{

    if args.observer.is_none() {
        let observer_trait = Path::from_string(MODEL_OBSERVER_TRAIT_PATH).unwrap();
        let model_name = &__struct.ident;
        let (impl_generics, type_generics, where_generics) = prepare_generics(&__struct.generics);
        return quote!(
          impl #impl_generics #observer_trait<#model_name #type_generics>  for #model_name #where_generics {}
        );
    }
    quote!()
}

/// this function determines that the attribute a custom attribute means
/// must remove it and replace it with something else
fn is_custom_attribute(attr: &Attribute) -> bool {
    let custom_attributes = [
        "no_default",
        "index"
    ];

    return custom_attributes.iter().any(|c| {
        c == &attr.meta.to_token_stream().to_string()
    });
}


/// this function gets syn generics and add needed bounds
/// ex user define this struct :
/// ```rust
/// struct Data<T>{
///     inner: T
/// }
/// ```
/// this function converts struct to this
///
/// ```rust
/// struct Data<T>
/// where T: Unpin , T: Sync , T: Default
/// {
///     inner: T
/// }
/// ```
/// the bounds are hard coded in NEEDED_BOUNDS variable
///
fn prepare_generics(generics: &Generics) -> (ImplGenerics, TypeGenerics, proc_macro2::TokenStream) {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    const NEEDED_BOUNDS: [&str; 7] = [
        "spark_orm::DeserializeOwned",
        "spark_orm::Serialize",
        "Debug",
        "Unpin",
        "Sync",
        "Send",
        "Default"
    ];

    let mut bounds = if generics.where_clause.is_some() {
        quote!(
          #where_clause ,

      )
    } else {
        quote!(
            where
        )
    };


    generics.params.iter().for_each(|generic| {
        if let GenericParam::Type(ty) = generic {
            NEEDED_BOUNDS.iter().for_each(|bound| {
                let ident = &ty.ident;
                let bound = Path::from_string(bound).unwrap();
                bounds = quote!(
                        #bounds

                        #ident :  #bound ,
                    );
            })
        }
    });
    (impl_generics, ty_generics, bounds)
}

