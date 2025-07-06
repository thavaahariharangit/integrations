use std::fmt::Display;

/*
 * Copyright (C) 2016-2025 Yuriy Yarosh
 * All rights reserved.
 *
 * SPDX-License-Identifier: MPL-2.0
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
use proc_macro2::TokenStream;
use quote::quote;

use bincode::{Decode, Encode};

#[derive(Debug, Clone, Default, PartialEq, Encode, Decode)]
pub enum FieldType {
    #[default]
    String,
    Number,
    Decimal,
    Bool,
    Custom(String),
}

impl FieldType {
    pub fn to_rust_type_string(&self) -> String {
        match self {
            FieldType::String => "String".to_string(),
            FieldType::Number => "i64".to_string(),
            FieldType::Decimal => "bigdecimal::BigDecimal".to_string(),
            FieldType::Bool => "bool".to_string(),
            FieldType::Custom(name) => name.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
pub struct APIStruct {
    pub name: String,
    pub fields: Vec<APIStructField>, /* field, type, default value */
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
pub struct APIStructField {
    pub name: String,
    pub field_type: FieldType,
    pub default_value: Option<String>,
}

impl APIStructField {
    pub fn new(name: String, field_type: FieldType, default_value: Option<String>) -> Self {
        Self {
            name,
            field_type,
            default_value,
        }
    }
}

impl APIStruct {
    pub fn new(name: String, fields: Vec<APIStructField>) -> Self {
        Self { name, fields }
    }

    pub fn to_stream(&self) -> TokenStream {
        let name = syn::Ident::new(&self.name, proc_macro2::Span::call_site());

        let struct_fields = self.fields.iter().map(|field| {
            let name = syn::Ident::new(&field.name, proc_macro2::Span::call_site());
            let type_string = field.field_type.to_rust_type_string();
            let ty: syn::Type = syn::parse_str(&type_string).unwrap();
            quote! { pub #name: #ty }
        });

        let struct_definition = quote! {
            #[derive(Debug, PartialEq)]
            pub struct #name {
                #(#struct_fields),*
            }
        };

        let has_default = self
            .fields
            .iter()
            .all(|field| field.default_value.is_some());
        if !has_default {
            return quote! {
                #struct_definition
            };
        }

        let default_fields = self.fields.iter().map(|field| {
            let name = syn::Ident::new(&field.name, proc_macro2::Span::call_site());
            let value = match &field.default_value {
                Some(value) => match field.field_type {
                    FieldType::String => quote! { #value.to_string() },
                    FieldType::Number => {
                        let val_lit: syn::LitInt = syn::parse_str(value).unwrap();
                        quote! { #val_lit }
                    }
                    FieldType::Decimal => {
                        quote! { ::std::str::FromStr::from_str(#value).unwrap() }
                    }
                    FieldType::Bool => {
                        let val_lit: syn::LitBool = syn::parse_str(value).unwrap();
                        quote! { #val_lit }
                    }
                    FieldType::Custom(_) => {
                        let val_path: syn::Path = syn::parse_str(value).unwrap();
                        quote! { #val_path }
                    }
                },
                None => quote! { Default::default() },
            };
            quote! { #name: #value }
        });

        let default_impl = quote! {
            impl Default for #name {
                fn default() -> Self {
                    Self {
                        #(#default_fields),*
                    }
                }
            }
        };

        quote! {
            #struct_definition
            #default_impl
        }
    }
}

impl Display for APIStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let file: syn::File = syn::parse2(self.to_stream()).unwrap();
        let code = prettyplease::unparse(&file);
        write!(f, "{code}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_struct_to_string() {
        let api_struct = APIStruct::new(
            "MyStruct".to_string(),
            vec![
                APIStructField::new("field1".to_string(), FieldType::String, None),
                APIStructField::new("field2".to_string(), FieldType::Number, None),
                APIStructField::new("field3".to_string(), FieldType::Decimal, None),
                APIStructField::new("field4".to_string(), FieldType::Bool, None),
                APIStructField::new(
                    "field5".to_string(),
                    FieldType::Custom("MyEnum".to_string()),
                    None,
                ),
            ],
        );

        goldie::assert!(api_struct.to_string());
    }

    #[test]
    fn test_struct_with_defaults_to_string() {
        let api_struct = APIStruct::new(
            "MyStructWithDefaults".to_string(),
            vec![
                APIStructField::new(
                    "field1".to_string(),
                    FieldType::String,
                    Some("default_string".to_string()),
                ),
                APIStructField::new(
                    "field2".to_string(),
                    FieldType::Number,
                    Some("42".to_string()),
                ),
                APIStructField::new(
                    "field3".to_string(),
                    FieldType::Decimal,
                    Some("123.45".to_string()),
                ),
                APIStructField::new(
                    "field4".to_string(),
                    FieldType::Bool,
                    Some("true".to_string()),
                ),
                APIStructField::new(
                    "field5".to_string(),
                    FieldType::Custom("MyEnum".to_string()),
                    Some("MyEnum::VariantA".to_string()),
                ),
                APIStructField::new("field6".to_string(), FieldType::String, None),
            ],
        );

        goldie::assert!(api_struct.to_string());
    }
}
