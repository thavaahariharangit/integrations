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
    pub fields: Vec<(String, FieldType)>,
}

impl APIStruct {
    pub fn new(name: String, fields: Vec<(String, FieldType)>) -> Self {
        Self { name, fields }
    }

    pub fn to_stream(&self) -> TokenStream {
        let name = syn::Ident::new(&self.name, proc_macro2::Span::call_site());

        let fields = self.fields.iter().map(|(field_name, field_type)| {
            let name = syn::Ident::new(field_name, proc_macro2::Span::call_site());
            let type_string = field_type.to_rust_type_string();
            let ty: syn::Type = syn::parse_str(&type_string).unwrap();
            quote! { pub #name: #ty }
        });

        quote! {
            pub struct #name {
                #(#fields),*
            }
        }
    }

    pub fn to_string(&self) -> String {
        let file: syn::File = syn::parse2(self.to_stream()).unwrap();
        prettyplease::unparse(&file)
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
                ("field1".to_string(), FieldType::String),
                ("field2".to_string(), FieldType::Number),
                ("field3".to_string(), FieldType::Decimal),
                ("field4".to_string(), FieldType::Bool),
                (
                    "field5".to_string(),
                    FieldType::Custom("MyEnum".to_string()),
                ),
            ],
        );

        goldie::assert!(api_struct.to_string());
    }
}
