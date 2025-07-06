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

use super::FieldType;

#[derive(Debug, Clone, Default, PartialEq, Encode, Decode)]
pub struct APIEnum {
    pub name: String,
    pub variants: Vec<(String, String)>,
    pub variant_type: FieldType,
}

impl APIEnum {
    pub fn new(name: String, variants: Vec<(String, String)>, variant_type: FieldType) -> Self {
        Self {
            name,
            variants,
            variant_type,
        }
    }

    pub fn to_stream(&self) -> TokenStream {
        let name = syn::Ident::new(&self.name, proc_macro2::Span::call_site());

        match self.variant_type {
            FieldType::Number => {
                let variants = self.variants.iter().map(|(variant_name, variant_value)| {
                    let ident = syn::Ident::new(variant_name, proc_macro2::Span::call_site());
                    let value_tokens: TokenStream = variant_value.parse().unwrap();
                    quote! { #ident = #value_tokens }
                });

                quote! {
                    pub enum #name {
                        #(#variants),*
                    }
                }
            }
            FieldType::String => {
                let variants = self.variants.iter().map(|(variant_name, variant_value)| {
                    let ident = syn::Ident::new(variant_name, proc_macro2::Span::call_site());
                    let lit_str = syn::LitStr::new(variant_value, proc_macro2::Span::call_site());
                    quote! { #ident = #lit_str }
                });
                quote! {
                    pub enum #name {
                        #(#variants),*
                    }
                }
            }
            FieldType::Bool => {
                let variants = self.variants.iter().map(|(variant_name, variant_value)| {
                    let ident = syn::Ident::new(variant_name, proc_macro2::Span::call_site());
                    let lit_bool = syn::LitBool::new(
                        variant_value.parse::<bool>().unwrap(),
                        proc_macro2::Span::call_site(),
                    );
                    quote! { #ident = #lit_bool }
                });
                quote! {
                    pub enum #name {
                        #(#variants),*
                    }
                }
            }
            _ => {
                let variants = self.variants.iter().map(|(variant_name, _)| {
                    let ident = syn::Ident::new(variant_name, proc_macro2::Span::call_site());
                    quote! { #ident }
                });

                let (return_type, method_name, value_mapper, uses): (
                    TokenStream,
                    syn::Ident,
                    Box<dyn Fn(&String) -> TokenStream>,
                    TokenStream,
                ) = match self.variant_type {
                    FieldType::Bool => (
                        quote! { bool },
                        syn::Ident::new("as_bool", proc_macro2::Span::call_site()),
                        Box::new(|v: &String| v.parse::<TokenStream>().unwrap()),
                        quote! {},
                    ),
                    FieldType::Decimal => (
                        quote! { BigDecimal },
                        syn::Ident::new("as_bigdecimal", proc_macro2::Span::call_site()),
                        Box::new(|v: &String| {
                            let value_with_new =
                                format!("BigDecimal::from_str(\"{}\").unwrap()", v);
                            value_with_new.parse().unwrap()
                        }),
                        quote! { use bigdecimal::BigDecimal; use std::str::FromStr; },
                    ),
                    _ => unreachable!(),
                };

                let match_arms = self.variants.iter().map(|(variant_name, variant_value)| {
                    let ident = syn::Ident::new(variant_name, proc_macro2::Span::call_site());
                    let value = value_mapper(variant_value);
                    quote! { Self::#ident => #value }
                });

                quote! {
                    #uses

                    pub enum #name {
                        #(#variants),*
                    }

                    impl #name {
                        pub fn #method_name(&self) -> #return_type {
                            match self {
                                #(#match_arms),*
                            }
                        }
                    }

                    impl Into<#return_type> for #name {
                        fn into(self) -> #return_type {
                            self.#method_name()
                        }
                    }
                }
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
    fn test_number_enum_to_string() {
        // Number
        let api_enum_number = APIEnum::new(
            "MyNumberEnum".to_string(),
            vec![
                ("VAR1".to_string(), "1".to_string()),
                ("VAR2".to_string(), "2".to_string()),
            ],
            FieldType::Number,
        );
        goldie::assert!(api_enum_number.to_string());
    }

    #[test]
    fn test_string_enum_to_string() {
        // String
        let api_enum_string = APIEnum::new(
            "MyStringEnum".to_string(),
            vec![
                ("VAR1".to_string(), "value1".to_string()),
                ("VAR2".to_string(), "value2".to_string()),
            ],
            FieldType::String,
        );
        goldie::assert!(api_enum_string.to_string());
    }

    #[test]
    fn test_bool_enum_to_string() {
        // Bool
        let api_enum_bool = APIEnum::new(
            "MyBoolEnum".to_string(),
            vec![
                ("VAR1".to_string(), "true".to_string()),
                ("VAR2".to_string(), "false".to_string()),
            ],
            FieldType::Bool,
        );
        goldie::assert!(api_enum_bool.to_string());
    }

    #[test]
    fn test_dec_enum_to_string() {
        // Decimal
        let api_enum_decimal = APIEnum::new(
            "MyDecimalEnum".to_string(),
            vec![
                ("VAR1".to_string(), "1.23".to_string()),
                ("VAR2".to_string(), "4.56".to_string()),
            ],
            FieldType::Decimal,
        );
        goldie::assert!(api_enum_decimal.to_string());
    }
}
