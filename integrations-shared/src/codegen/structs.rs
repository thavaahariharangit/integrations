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

/// Represents the type of a field in an API struct.
#[derive(Debug, Clone, Default, PartialEq, Encode, Decode)]
pub enum FieldType {
    #[default]
    String,
    Number,
    Decimal,
    Bool,
    Struct(String),
    Enum(String),
}

impl FieldType {
    /// Returns the Rust type as a string for this field type.
    pub fn to_rust_type_string(&self) -> String {
        match self {
            FieldType::String => "String".to_string(),
            FieldType::Number => "i64".to_string(),
            FieldType::Decimal => "bigdecimal::BigDecimal".to_string(),
            FieldType::Bool => "bool".to_string(),
            FieldType::Struct(name) => name.clone(),
            FieldType::Enum(name) => name.clone(),
        }
    }
}

/// Represents an API struct with a name and a list of fields.
#[derive(Debug, Clone, PartialEq, Encode, Decode)]
pub struct APIStruct {
    pub name: String,
    pub fields: Vec<APIStructField>,
}

/// Represents a single field in an API struct.
#[derive(Debug, Clone, PartialEq, Encode, Decode)]
pub struct APIStructField {
    pub name: String,
    pub field_type: FieldType,
    pub default_value: Option<String>,

    pub is_array: bool,
    pub is_string_map: bool,

    pub min: Option<i64>,
    pub max: Option<i64>,
}

impl APIStructField {
    /// Creates a new APIStructField.
    pub fn new(
        name: String,
        field_type: FieldType,
        default_value: Option<String>,
        is_array: bool,
        is_string_map: bool,
        min: Option<i64>,
        max: Option<i64>,
    ) -> Self {
        Self {
            name,
            field_type,
            default_value,
            is_array,
            is_string_map,
            min,
            max,
        }
    }

    pub fn to_stream(&self, api_endpoint: Option<&super::APIEndpoint>) -> TokenStream {
        let name = syn::Ident::new(&self.name, proc_macro2::Span::call_site());
        let mut type_string = self.field_type.to_rust_type_string();
        if self.is_array {
            type_string = format!("Vec<{}>", type_string);
        } else if self.is_string_map {
            type_string = format!("std::collections::HashMap<String, {}>", type_string);
        }
        let ty: syn::Type = syn::parse_str(&type_string).unwrap();
        let mut attributes: Vec<TokenStream> = Vec::new();
        // Validation attributes
        if self.is_array || self.is_string_map {
            if let Some(min) = self.min {
                attributes.push(quote! { min_items = #min });
            }
            if let Some(max) = self.max {
                attributes.push(quote! { max_items = #max });
            }
        } else if self.field_type == FieldType::String {
            if let Some(min) = self.min {
                attributes.push(quote! { min_length = #min });
            }
            if let Some(max) = self.max {
                attributes.push(quote! { max_length = #max });
            }
        } else if self.field_type == FieldType::Number || self.field_type == FieldType::Decimal {
            if let Some(min) = self.min {
                attributes.push(quote! { min = #min });
            }
            if let Some(max) = self.max {
                attributes.push(quote! { max = #max });
            }
        }
        // Enum validation
        if let FieldType::Enum(enum_name) = &self.field_type {
            if let Some(api_endpoint) = api_endpoint {
                if let Some(e) = api_endpoint.enum_by_name(enum_name) {
                    let valid_variants: Vec<_> = e
                        .variants
                        .iter()
                        .filter_map(|(_, variant_value, _)| match e.variant_type {
                            FieldType::String => Some(quote! { #variant_value }),
                            FieldType::Number => {
                                let val: syn::LitInt = syn::parse_str(variant_value).unwrap();
                                Some(quote! { #val })
                            }
                            FieldType::Decimal => {
                                let val: syn::LitFloat = syn::parse_str(variant_value).unwrap();
                                Some(quote! { #val })
                            }
                            _ => None,
                        })
                        .collect();
                    if !valid_variants.is_empty() {
                        attributes.push(quote! { enumerate = [#(#valid_variants),*] });
                    }
                }
            }
        }
        let validation_attr = if !attributes.is_empty() {
            quote! { #[validate(#(#attributes),*)] }
        } else {
            quote! {}
        };
        quote! {
            #validation_attr
            pub #name: #ty
        }
    }
}

impl APIStruct {
    pub fn new(name: String, fields: Vec<APIStructField>) -> Self {
        Self { name, fields }
    }


    fn has_validation(
        fields: &[APIStructField],
        api_endpoint: Option<&super::APIEndpoint>,
    ) -> bool {
        fields.iter().any(|field| {
            if field.min.is_some() || field.max.is_some() {
                return true;
            }
            if let FieldType::Enum(enum_name) = &field.field_type {
                if let Some(endpoint) = api_endpoint {
                    if let Some(e) = endpoint.enum_by_name(enum_name) {
                        return !e.variants.is_empty()
                            && matches!(
                                e.variant_type,
                                FieldType::String | FieldType::Number | FieldType::Decimal
                            );
                    }
                }
            }
            false
        })
    }

    /// Helper: Generates the struct definition TokenStream.
    fn struct_definition_tokens(
        name: &syn::Ident,
        struct_fields: &[TokenStream],
        has_validate: bool,
    ) -> TokenStream {
        if has_validate {
            quote! {
                #[derive(Debug, PartialEq, Validate)]
                pub struct #name {
                    #(#struct_fields),*
                }
            }
        } else {
            quote! {
                #[derive(Debug, PartialEq)]
                pub struct #name {
                    #(#struct_fields),*
                }
            }
        }
    }

    /// Helper: Generates the default value TokenStream for a field.
    fn default_field_tokens(field: &APIStructField) -> TokenStream {
        let name = syn::Ident::new(&field.name, proc_macro2::Span::call_site());
        let value = field.default_value.as_ref().unwrap();
        let value_stream = match &field.field_type {
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
            FieldType::Struct(_) | FieldType::Enum(_) => {
                let val_path: syn::Path = syn::parse_str(value).unwrap();
                quote! { #val_path }
            }
        };
        quote! { #name: #value_stream }
    }


    pub fn to_stream(&self, api_endpoint: Option<&super::APIEndpoint>) -> TokenStream {
        let name = syn::Ident::new(&self.name, proc_macro2::Span::call_site());
        let struct_fields = self
            .fields
            .iter()
            .map(|field| field.to_stream(api_endpoint))
            .collect::<Vec<_>>();
        let has_validate = Self::has_validation(&self.fields, api_endpoint);
        let struct_definition = Self::struct_definition_tokens(&name, &struct_fields, has_validate);
        let has_full_default = self
            .fields
            .iter()
            .all(|field| field.default_value.is_some());
        if !has_full_default {
            return quote! {
                #struct_definition
            };
        }
        let default_fields = self.fields.iter().map(Self::default_field_tokens);
        quote! {
            #struct_definition
            impl Default for #name {
                fn default() -> Self {
                    Self {
                        #(#default_fields),*
                    }
                }
            }
        }
    }
}

impl Display for APIStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stream = self.to_stream(None);
        let file: syn::File = syn::parse2(stream).unwrap();
        let code = prettyplease::unparse(&file);
        write!(f, "{code}")
    }
}

#[cfg(test)]
mod tests {
    use crate::codegen::{APIEndpoint, APIEnum};

    use super::*;

    fn create_field(
        name: &str,
        field_type: FieldType,
        default_value: Option<&str>,
        is_array: bool,
        is_string_map: bool,
        min: Option<i64>,
        max: Option<i64>,
    ) -> APIStructField {
        APIStructField::new(
            name.to_string(),
            field_type,
            default_value.map(String::from),
            is_array,
            is_string_map,
            min,
            max,
        )
    }

    #[test]
    fn test_basic_struct() {
        let api_struct = APIStruct::new(
            "MyStruct".to_string(),
            vec![
                create_field(
                    "field_string",
                    FieldType::String,
                    None,
                    false,
                    false,
                    None,
                    None,
                ),
                create_field(
                    "field_number",
                    FieldType::Number,
                    None,
                    false,
                    false,
                    None,
                    None,
                ),
                create_field(
                    "field_decimal",
                    FieldType::Decimal,
                    None,
                    false,
                    false,
                    None,
                    None,
                ),
                create_field(
                    "field_bool",
                    FieldType::Bool,
                    None,
                    false,
                    false,
                    None,
                    None,
                ),
                create_field(
                    "field_custom",
                    FieldType::Enum("MyEnum".to_string()),
                    None,
                    false,
                    false,
                    None,
                    None,
                ),
            ],
        );

        goldie::assert!(&api_struct.to_string());
    }

    #[test]
    fn test_struct_with_validation() {
        let api_struct = APIStruct::new(
            "MyStructWithValidation".to_string(),
            vec![
                create_field(
                    "string_with_len",
                    FieldType::String,
                    None,
                    false,
                    false,
                    Some(1),
                    Some(10),
                ),
                create_field(
                    "number_with_range",
                    FieldType::Number,
                    None,
                    false,
                    false,
                    Some(0),
                    Some(100),
                ),
                create_field(
                    "decimal_with_range",
                    FieldType::Decimal,
                    None,
                    false,
                    false,
                    Some(0),
                    Some(100),
                ),
                create_field(
                    "array_with_size",
                    FieldType::String,
                    None,
                    true,
                    false,
                    Some(1),
                    Some(5),
                ),
                create_field(
                    "map_with_size",
                    FieldType::Number,
                    None,
                    false,
                    true,
                    Some(1),
                    Some(10),
                ),
            ],
        );

        goldie::assert!(&api_struct.to_string());
    }

    #[test]
    fn test_struct_with_collections() {
        let api_struct = APIStruct::new(
            "MyStructWithCollections".to_string(),
            vec![
                create_field(
                    "string_array",
                    FieldType::String,
                    None,
                    true,
                    false,
                    None,
                    None,
                ),
                create_field(
                    "custom_map",
                    FieldType::Struct("AnotherType".to_string()),
                    None,
                    false,
                    true,
                    None,
                    None,
                ),
            ],
        );

        goldie::assert!(&api_struct.to_string());
    }

    #[test]
    fn test_struct_with_full_defaults() {
        let api_struct = APIStruct::new(
            "MyStructWithDefaults".to_string(),
            vec![
                create_field(
                    "field1",
                    FieldType::String,
                    Some("default_string"),
                    false,
                    false,
                    None,
                    None,
                ),
                create_field(
                    "field2",
                    FieldType::Number,
                    Some("42"),
                    false,
                    false,
                    None,
                    None,
                ),
                create_field(
                    "field3",
                    FieldType::Decimal,
                    Some("123.45"),
                    false,
                    false,
                    None,
                    None,
                ),
                create_field(
                    "field4",
                    FieldType::Bool,
                    Some("true"),
                    false,
                    false,
                    None,
                    None,
                ),
                create_field(
                    "field5",
                    FieldType::Enum("MyEnum".to_string()),
                    Some("MyEnum::VariantA"),
                    false,
                    false,
                    None,
                    None,
                ),
            ],
        );

        goldie::assert!(&api_struct.to_string());
    }

    #[test]
    fn test_struct_with_partial_defaults() {
        let api_struct = APIStruct::new(
            "MyStructWithPartialDefaults".to_string(),
            vec![
                create_field(
                    "field1",
                    FieldType::String,
                    Some("default_string"),
                    false,
                    false,
                    None,
                    None,
                ),
                create_field("field2", FieldType::Number, None, false, false, None, None),
            ],
        );

        goldie::assert!(&api_struct.to_string());
    }

    #[test]
    fn test_struct_with_enum_validation() {
        let api_endpoint = APIEndpoint::new(
            "MyEndpoint".to_string(),
            "/my-endpoint".to_string(),
            crate::codegen::APIMethod::GET,
            false,
            vec![],
            vec![APIEnum::new(
                "MyTestEnum".to_string(),
                vec![("VariantA".to_string(), "A".to_string(), false), ("VariantB".to_string(), "B".to_string(), false)],
                FieldType::String,
            )],
            None,
            None,
            None,
        );

        let api_struct = APIStruct::new(
            "MyStructWithEnumValidation".to_string(),
            vec![create_field(
                "enum_field",
                FieldType::Enum("MyTestEnum".to_string()),
                None,
                false,
                false,
                None,
                None,
            )],
        );

        let file: syn::File = syn::parse2(api_struct.to_stream(Some(&api_endpoint))).unwrap();
        let code = prettyplease::unparse(&file);
        goldie::assert!(&code);
    }

    #[test]
    fn test_struct_with_numeric_enum_validation() {
        let api_endpoint = APIEndpoint::new(
            "MyEndpoint".to_string(),
            "/my-endpoint".to_string(),
            crate::codegen::APIMethod::GET,
            false,
            vec![],
            vec![APIEnum::new(
                "MyNumericEnum".to_string(),
                vec![("VariantA".to_string(), "1".to_string(), false), ("VariantB".to_string(), "2".to_string(), false)],
                FieldType::Number,
            )],
            None,
            None,
            None,
        );

        let api_struct = APIStruct::new(
            "MyStructWithNumericEnumValidation".to_string(),
            vec![create_field(
                "enum_field",
                FieldType::Enum("MyNumericEnum".to_string()),
                None,
                false,
                false,
                None,
                None,
            )],
        );

        let file: syn::File = syn::parse2(api_struct.to_stream(Some(&api_endpoint))).unwrap();
        let code = prettyplease::unparse(&file);
        goldie::assert!(&code);
    }
}
