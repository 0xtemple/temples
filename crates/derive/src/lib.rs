#![no_std]

use gstd::Vec;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields};

#[proc_macro_derive(StorageValue)]
pub fn storage_value(input: TokenStream) -> TokenStream {
    env_logger::init();
    // Parse the input tokens into a syntax tree
    let input = syn::parse_macro_input!(input as DeriveInput);

    // Get the name of the struct
    let struct_name = &input.ident;

    // Get fields of the struct
    let fields = match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => &fields.named,
            _ => {
                panic!("DebugPrint only supports named fields in structs");
            }
        },
        _ => {
            panic!("DebugPrint only supports structs");
        }
    };

    // Get field names and types and convert them to strings
    let (field_names, field_types) = fields
        .iter()
        .map(|field| {
            let field_name = field.ident.as_ref().expect("Expected field name");
            let field_type = &field.ty;
            (
                quote! { stringify!(#field_name).to_string() },
                quote! { stringify!(#field_type).to_string() },
            )
        })
        .unzip::<_, _, Vec<quote::__private::TokenStream>, Vec<quote::__private::TokenStream>>();

    let gen = quote! {
        impl #struct_name {
            pub fn component_id() -> [u8;32] {
                let mut component_id = vec![0,0];
                let mut schema_name_bytes = stringify!(#struct_name).as_bytes().to_vec();
                schema_name_bytes.resize(30, 0);
                component_id.extend(&schema_name_bytes);
                let component_id:[u8;32] = component_id.try_into().expect("Failed");
                component_id
            }

            pub fn metadata() -> ComponentMetadata  {
                ComponentMetadata {
                    name: stringify!(#struct_name).to_string(),
                    key_names: vec![],
                    key_types: vec![],
                    value_names: vec![#(#field_names),*],
                    value_types: vec![#(#field_types),*],
                    ty: temple_types::component::ComponentType::Onchain,
                }
            }

             pub fn register() {
                let temple_world_storage =
                    unsafe { temple_storage::io::TEMPLE_WORLD_STORAGE.get_or_insert(Default::default()) };
                let metadata = Self::metadata();
                temple_world_storage.register_component(&metadata.name, &metadata);
                temple_storage::emitter::emit_register_event(
                    Self::component_id(),
                    metadata.key_names,
                    metadata.key_types,
                    metadata.value_names,
                    metadata.value_types,
                )
            }

            pub fn get() -> Self {
                let temple_storage_value =
                    unsafe { temple_storage::io::TEMPLE_STORAGE_VALUE.get_or_insert(Default::default()) };
                Self::decode(
                    &mut &temple_storage_value
                        .get(&Self::component_id())
                        .unwrap_or(&Vec::new())[..],
                ).unwrap_or(Default::default())
            }

            pub fn set(value: Self) {
                let temple_storage_value =
                    unsafe { temple_storage::io::TEMPLE_STORAGE_VALUE.get_or_insert(Default::default()) };
                let component_id = Self::component_id();
                let component_data = value.encode();
                temple_storage_value.insert(component_id, component_data.clone());
                temple_storage::emitter::emit_set_event(component_id, vec![], component_data);
            }
        }
    };
    gen.into()
}

// #[proc_macro_derive(StorageValue,attributes(key))]
// pub fn storage_value(input: TokenStream) -> TokenStream {
//     env_logger::init();
//     // Parse the input tokens into a syntax tree
//     let input = syn::parse_macro_input!(input as DeriveInput);
//
//     // Get the name of the struct
//     let struct_name = &input.ident;
//
//     // Get fields of the struct
//     let fields = match input.data {
//         Data::Struct(ref data) => match data.fields {
//             Fields::Named(ref fields) => &fields.named,
//             _ => {
//                 panic!("DebugPrint only supports named fields in structs");
//             }
//         },
//         _ => {
//             panic!("DebugPrint only supports structs");
//         }
//     };
//     let field_prints = fields.iter().map(|field| {
//         let field_name = field.ident.as_ref().expect("Expected field name");
//         let field_type = &field.ty;
//         quote! {
//             println!("Field {}: {:?}", stringify!(#field_name), stringify!(#field_type));
//         }
//     });
//
//     // Get field names with #[key] attribute
//     let key_field_names = fields
//         .iter()
//         .filter_map(|field| {
//             for attr in &field.attrs {
//                 if attr.path().is_ident("key") {
//                     return field.ident.as_ref().map(|ident| ident.to_string());
//                 }
//             }
//             None
//         })
//         .collect::<Vec<String>>();
//     debug!("-------{:?}", key_field_names);
//
//
//
//     // Get field names and types and convert them to strings
//     let (field_names, field_types) = fields
//         .iter()
//         .map(|field| {
//             let field_name = field.ident.as_ref().expect("Expected field name");
//             let field_type = &field.ty;
//             (quote! { stringify!(#field_name).to_string() }, quote! { stringify!(#field_type).to_string() })
//         })
//         .unzip::<_, _, Vec<quote::__private::TokenStream>, Vec<quote::__private::TokenStream>>();
//
//     let gen = quote! {
//         impl StorageValue for #struct_name {
//             fn hello_macro() {
//                 println!("{:?}", stringify!(#struct_name).encode());
//                 println!("Hello, Macro! My name is {}!", stringify!(#struct_name));
//                  #( #field_prints )*
//             }
//
//             fn component_id() -> [u8;32] {
//                 let mut component_id = vec![0,0];
//                 let mut schema_name_bytes = stringify!(#struct_name).as_bytes().to_vec();
//                 schema_name_bytes.resize(30, 0);
//                 component_id.extend(&schema_name_bytes);
//                 let component_id:[u8;32] = component_id.try_into().expect("Failed");
//                 component_id
//             }
//
//             fn metadata() -> ComponentMetadata  {
//                    println!("{:?}", stringify!(#struct_name).to_string());
//                 println!("{:?}", vec![#(#field_names),*]);
//                 println!("{:?}", vec![#(#field_types),*]);
//
//                 ComponentMetadata {
//                     name: stringify!(#struct_name).to_string(),
//                     key_names: vec![],
//                     key_types: vec![],
//                     value_names: vec![#(#field_names),*],
//                     value_types: vec![#(#field_types),*],
//                     ty: temple_types::component::ComponentType::Onchain,
//                 }
//             }
//
//              fn get_key_fields() -> Vec<String> {
//                 vec![#((#key_field_names).to_string()),*]
//                 // #key_field_names
//             }
//
//             pub fn get(#(#key_field_names: String),*) {
//                 println!("{:?}", #(#key_field_names),*);
//             }
//         }
//     };
//     gen.into()
//
// }
//
// fn has_key_attribute(attrs: &Vec<Attribute>) -> bool {
//     attrs.iter().any(|attr| {
//         debug!("-------{:?}", attr.path());
//         if  attr.path().is_ident("key") {
//             return true;
//         }
//         false
//     })
// }
