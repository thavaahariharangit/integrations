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

use std::fmt::Display;

use proc_macro2::TokenStream;
use quote::quote;

use bincode::{Decode, Encode};

use super::FieldType;

#[derive(Debug, Clone, Default, PartialEq, Encode, Decode)]
pub struct APIEnum {
    pub name: String,
    pub variants: Vec<(String, String, bool)>, /* name, value, is_default */
    pub variant_type: FieldType,
}

impl APIEnum {
    pub fn new(
        name: String,
        variants: Vec<(String, String, bool)>,
        variant_type: FieldType,
    ) -> Self {
        Self {
            name,
            variants,
            variant_type,
        }
    }

    pub fn to_stream(&self) -> TokenStream {
        let name = syn::Ident::new(&self.name, proc_macro2::Span::call_site());

        let has_default = self.variants.iter().any(|(_, _, is_default)| *is_default);
        let derives = if has_default {
            quote! { #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)] }
        } else {
            quote! { #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] }
        };

        match self.variant_type {
            FieldType::Number => {
                let variants =
                    self.variants
                        .iter()
                        .map(|(variant_name, variant_value, is_default)| {
                            let ident =
                                syn::Ident::new(variant_name, proc_macro2::Span::call_site());
                            let value_tokens: TokenStream = variant_value.parse().unwrap();
                            let default_attr = if *is_default {
                                quote! { #[default] }
                            } else {
                                quote! {}
                            };
                            quote! { #default_attr #ident = #value_tokens }
                        });

                quote! {
                    #derives
                    pub enum #name {
                        #(#variants),*
                    }
                }
            }
            FieldType::String => {
                let variants = self.variants.iter().map(|(variant_name, _, is_default)| {
                    let ident = syn::Ident::new(variant_name, proc_macro2::Span::call_site());
                    let default_attr = if *is_default {
                        quote! { #[default] }
                    } else {
                        quote! {}
                    };
                    quote! { #default_attr #ident }
                });

                let from_variants = self
                    .variants
                    .iter()
                    .map(|(variant_name, variant_value, _)| {
                        let ident = syn::Ident::new(variant_name, proc_macro2::Span::call_site());
                        quote! {
                            #name::#ident => #variant_value.to_string(),
                        }
                    });

                quote! {
                    #derives
                    pub enum #name {
                        #(#variants),*
                    }

                    impl From<#name> for String {
                        fn from(variant: #name) -> Self {
                            match variant {
                                #(#from_variants)*
                            }
                        }
                    }
                }
            }
            FieldType::Bool => {
                let variants =
                    self.variants
                        .iter()
                        .map(|(variant_name, variant_value, is_default)| {
                            let ident =
                                syn::Ident::new(variant_name, proc_macro2::Span::call_site());
                            let lit_bool = syn::LitBool::new(
                                variant_value.parse::<bool>().unwrap(),
                                proc_macro2::Span::call_site(),
                            );
                            let default_attr = if *is_default {
                                quote! { #[default] }
                            } else {
                                quote! {}
                            };
                            quote! { #default_attr #ident = #lit_bool }
                        });
                quote! {
                    #derives
                    pub enum #name {
                        #(#variants),*
                    }
                }
            }
            FieldType::Decimal => {
                let variants = self.variants.iter().map(|(variant_name, _, is_default)| {
                    let ident = syn::Ident::new(variant_name, proc_macro2::Span::call_site());
                    let default_attr = if *is_default {
                        quote! { #[default] }
                    } else {
                        quote! {}
                    };
                    quote! { #default_attr #ident }
                });

                let match_arms = self
                    .variants
                    .iter()
                    .map(|(variant_name, variant_value, _)| {
                        let ident = syn::Ident::new(variant_name, proc_macro2::Span::call_site());
                        let value = format!("BigDecimal::from_str(\"{variant_value}\").unwrap()");
                        let value_tokens: TokenStream = value.parse().unwrap();
                        quote! { Self::#ident => #value_tokens }
                    });

                quote! {
                    #derives
                    pub enum #name {
                        #(#variants),*
                    }

                    impl #name {
                        pub fn as_bigdecimal(&self) -> BigDecimal {
                            match self {
                                #(#match_arms),*
                            }
                        }
                    }

                    impl From<#name> for BigDecimal {
                        fn from(val: #name) -> Self {
                            val.as_bigdecimal()
                        }
                    }
                }
            }
            FieldType::Struct(_) | FieldType::Enum(_) => {
                // Enums cannot have variants of type Struct or Enum in this model.
                quote! {}
            }
        }
    }
}

impl Display for APIEnum {
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
    fn test_number_enum_to_string() {
        // Number
        let api_enum_number = APIEnum::new(
            "MyNumberEnum".to_string(),
            vec![
                ("VAR1".to_string(), "1".to_string(), false),
                ("VAR2".to_string(), "2".to_string(), false),
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
                ("VAR1".to_string(), "value1".to_string(), false),
                ("VAR2".to_string(), "value2".to_string(), false),
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
                ("VAR1".to_string(), "true".to_string(), false),
                ("VAR2".to_string(), "false".to_string(), false),
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
                ("VAR1".to_string(), "1.23".to_string(), false),
                ("VAR2".to_string(), "4.56".to_string(), false),
            ],
            FieldType::Decimal,
        );
        goldie::assert!(api_enum_decimal.to_string());
    }

    #[test]
    fn test_enum_with_default_to_string() {
        let api_enum = APIEnum::new(
            "MyEnumWithDefault".to_string(),
            vec![
                ("VAR1".to_string(), "1".to_string(), false),
                ("VAR2".to_string(), "2".to_string(), true),
            ],
            FieldType::Number,
        );

        goldie::assert!(api_enum.to_string());
    }
}
