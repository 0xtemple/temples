#![no_std]

extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec;
use gstd::Vec;
use parity_scale_codec::Encode;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::spanned::Spanned;
use syn::token::Struct;
use syn::{
    parse, parse_macro_input, Attribute, Data, DataStruct, DeriveInput, Fields, FieldsNamed, LitStr,
};
use temple_types::resource::{Resource, ResourceType};

#[proc_macro_derive(Schema, attributes(type_id, namespace, key))]
pub fn schema(input: TokenStream) -> TokenStream {
    env_logger::init();
    // Parse the input tokens into a syntax tree
    // let args = syn::parse_macro_input!(args as AttributeArgs);
    let input = syn::parse_macro_input!(input as DeriveInput);
    // log::debug!("{:#?}", input.attrs);

    // Get the name of the struct
    let struct_name = &input.ident;

    let struct_name_string = &input.ident.to_string();
    let (namespace_string, type_id_string) = parse_args(&input.attrs);

    let fields = if let Data::Struct(DataStruct {
        fields: Fields::Named(FieldsNamed { named: ref n, .. }),
        ..
    }) = input.data
    {
        n
    } else {
        panic!("StorageValue only supports structs");
    };

    let mut key_field_idents: Vec<_> = vec![];
    let mut key_field_tys: Vec<_> = vec![];
    let mut field_idents: Vec<_> = vec![];
    let mut field_tys: Vec<_> = vec![];

    fields.iter().for_each(|f| {
        let is_key = f.attrs.iter().any(|attr| attr.path.is_ident("key"));
        if is_key {
            key_field_idents.push(f.ident.as_ref().unwrap());
            key_field_tys.push(&f.ty);
        } else {
            field_idents.push(f.ident.as_ref().unwrap());
            field_tys.push(&f.ty);
        }
    });

    let mut key_names: Vec<_> = vec![];
    let mut key_types: Vec<_> = vec![];
    let mut field_names: Vec<_> = vec![];
    let mut field_types: Vec<_> = vec![];

    fields.iter().for_each(|f| {
        let is_key = f.attrs.iter().any(|attr| attr.path.is_ident("key"));
        let ident = f.ident.as_ref().unwrap();
        let ty = &f.ty;
        if is_key {
            key_names.push(quote! { stringify!(#ident).to_string() });
            key_types.push(quote! { stringify!(#ty).to_string() });
        } else {
            field_names.push(quote! { stringify!(#ident).to_string() });
            field_types.push(quote! { stringify!(#ty).to_string() });
        }
    });

    let mut gen = quote! {
        impl #struct_name {
            pub fn resource_id() -> [u8;32] {
                let mut type_id = temple_types::resource::ResourceType::from(#type_id_string);
                let resource_type = temple_types::resource::Resource::new(#namespace_string, #struct_name_string, type_id);
                resource_type.encode().try_into().expect("Failed to encode")
            }

            pub fn metadata() -> SchemaMetadata {
                SchemaMetadata {
                    key_names: vec![#(#key_names),*],
                    key_types: vec![#(#key_types),*],
                    value_names: vec![#(#field_names),*],
                    value_types: vec![#(#field_types),*],
                }
            }

             pub async fn register(nexcore: gstd::ActorId) {
                let resource_id = Self::resource_id();
                let schema_metadata = Self::metadata();
                gstd::msg::send_for_reply_as::<_, temple_types::event::NexCoreEvent>(
                    nexcore,
                    temple_types::action::SystemAction::RegisterSchema {
                0: resource_id,
                1: schema_metadata,
            },
            0,
            0,
        )
            .expect("Error in async message to Mtk contract")
            .await
            .expect("Error in async message to Mtk contract");
            }

            pub async fn get(nexcore: gstd::ActorId) -> Self {
                let result = gstd::msg::send_for_reply_as::<_, temple_types::event::NexCoreEvent>(
                    nexcore,
                    temple_types::action::SystemAction::GetRecord {
                        0: Self::resource_id(),
                        1: vec![],
                    },
                    0,
                    0,
                )
                    .expect("Error in async message to Mtk contract")
                    .await
                    .expect("CONCERT: Error getting balances from the contract");
                if let temple_types::event::NexCoreEvent::GetRecordSuccess(value) = result {
                    Self::decode(&mut &value[..]).expect("Decode failed")
                } else {
                    Self::default()
                }
            }

            pub async fn set(nexcore: gstd::ActorId, value: Self) {
                gstd::msg::send_for_reply_as::<_, temple_types::event::NexCoreEvent>(
                    nexcore,
                    temple_types::action::SystemAction::SetRecord {
                        0: Self::resource_id(),
                        1: vec![],
                        2: value.encode()
                    },
                    0,
                    0,
                )
                    .expect("Error in async message to Mtk contract")
                    .await
                    .expect("CONCERT: Error getting balances from the contract");
            }
        }
    };
    if !key_field_idents.is_empty() {
        gen = quote! {
            impl #struct_name {
                pub fn resource_id() -> [u8;32] {
                    let mut type_id = temple_types::resource::ResourceType::from(#type_id_string);
                    let resource_type = temple_types::resource::Resource::new(#namespace_string, #struct_name_string, type_id);
                    resource_type.encode().try_into().expect("Failed to encode")
                }

                pub fn metadata() -> SchemaMetadata {
                    SchemaMetadata {
                        key_names: vec![#(#key_names),*],
                        key_types: vec![#(#key_types),*],
                        value_names: vec![#(#field_names),*],
                        value_types: vec![#(#field_types),*],
                    }
                }

                 pub async fn register(nexcore: gstd::ActorId) {
                    let resource_id = Self::resource_id();
                    let schema_metadata = Self::metadata();
                    gstd::msg::send_for_reply_as::<_, temple_types::event::NexCoreEvent>(
                        nexcore,
                        temple_types::action::SystemAction::RegisterSchema {
                    0: resource_id,
                    1: schema_metadata,
                },
                0,
                0,
            )
                .expect("Error in async message to Mtk contract")
                .await
                .expect("Error in async message to Mtk contract");
                }

                pub async fn get(nexcore: gstd::ActorId, key: (#(#key_field_tys),*)) -> Self {
                    let result = gstd::msg::send_for_reply_as::<_, temple_types::event::NexCoreEvent>(
                        nexcore,
                        temple_types::action::SystemAction::GetRecord {
                            0: Self::resource_id(),
                            1: key.encode(),
                        },
                        0,
                        0,
                    )
                        .expect("Error in async message to Mtk contract")
                        .await
                        .expect("CONCERT: Error getting balances from the contract");
                    if let temple_types::event::NexCoreEvent::GetRecordSuccess(value) = result {
                        Self::decode(&mut &value[..]).expect("Decode failed")
                    } else {
                        Self::default()
                    }
                }

                pub async fn set(nexcore: gstd::ActorId, key: (#(#key_field_tys),*), value: Self) {
                    gstd::msg::send_for_reply_as::<_, temple_types::event::NexCoreEvent>(
                        nexcore,
                        temple_types::action::SystemAction::SetRecord {
                            0: Self::resource_id(),
                            1: key.encode(),
                            2: value.encode()
                        },
                        0,
                        0,
                    )
                        .expect("Error in async message to Mtk contract")
                        .await
                        .expect("CONCERT: Error getting balances from the contract");
                }
            }
        };
    }
    gen.into()
}

fn parse_args(attr: &Vec<Attribute>) -> (String, String) {
    let mut namespace = Ident::new("Root", attr.first().span());
    let mut type_id = Ident::new("Onchain", attr.first().span());
    attr.iter().for_each(|attr| {
        if let Some(ident) = attr.path.get_ident() {
            if ident == "type_id" {
                attr.tokens.to_token_stream().into_iter().for_each(|token| {
                    if let proc_macro2::TokenTree::Group(g) = token {
                        log::debug!("{}", g.stream().to_string());
                        let type_id_value = g.stream().to_string();
                        type_id = Ident::new(&type_id_value, g.span());
                    }
                });
            } else if ident == "namespace" {
                attr.tokens.to_token_stream().into_iter().for_each(|token| {
                    if let proc_macro2::TokenTree::Group(g) = token {
                        let namespace_value = g.stream().to_string();
                        namespace = Ident::new(&namespace_value, g.span());
                    }
                });
            }
        } else {
            panic!("The ident is not support.")
        }
    });
    (namespace.to_string(), type_id.to_string())
}
