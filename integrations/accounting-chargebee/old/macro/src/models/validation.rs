/*
    Copyright (C) 2017-2025 Yuriy Yarosh.
    All rights reserved.
*/
use crate::models::codegen;
use proc_macro2::{Ident, Span, TokenStream};
use quote::*;
use std::str::FromStr;
use syn::LitInt;

#[derive(Default, Debug, Clone)]
pub(crate) struct FieldValidation {
    /// Maximum length constraint for string types
    pub max_length: Option<usize>,
    /// Minimum length constraint for string types
    pub min_length: Option<usize>,
    /// Minimum value constraint for numeric types
    pub min: Option<i64>,
    /// Maximum value constraint for numeric types
    pub max: Option<i64>,
    /// Whether the field is optional
    pub optional: bool,
    /// Whether the field represents a monetary value in cents
    pub monetary_cents: bool,
    /// Default value for the field, if any
    pub default_value: Option<String>,
}

impl FieldValidation {
    pub(crate) fn swap_ranges(&self) -> FieldValidation {
        let max_length = self.max_length;
        let min_length = self.min_length;

        FieldValidation {
            max_length: self.max.map(|e| e as usize).clone(),
            min_length: self.min.map(|e| e as usize).clone(),
            max: max_length.map(|e| e as i64),
            min: min_length.map(|e| e as i64),

            optional: self.optional,
            monetary_cents: self.monetary_cents,
            default_value: self.default_value.clone(),
        }
    }

    pub(crate) fn parse(attr_spec: &str) -> codegen::Result<(FieldValidation, Vec<String>)> {
        let mut max_length = None;
        let mut min_length = None;
        let mut min = None;
        let mut max = None;
        let mut optional = false;
        let mut monetary_cents = false;
        let mut default_value = None;

        fn get_str_val(v: &str) -> String {
            v.split("=").last().unwrap().trim().to_string()
        }

        fn get_val<T: FromStr>(v: &str) -> T
        where
            T::Err: std::fmt::Debug,
        {
            let value = get_str_val(v);
            // Optional transformation for specific types, e.g., handling "k" as thousands for integers
            if value.contains('k')
                && (std::any::type_name::<T>() == "i64" || std::any::type_name::<T>() == "u64")
            {
                value.replace('k', "000").parse::<T>()
            } else {
                value.parse::<T>()
            }
            .expect(format!("can't parse {} {}", std::any::type_name::<T>(), value).as_str())
        }

        fn valid_const(s: &str) -> bool {
            let val = get_str_val(s);
            !val[1..].starts_with("999")
                && !s.to_lowercase().contains("1e")
                && !s.contains(".")
                && !s.contains(",") // skipping floats, 999999 and 1e9
        }

        let leftovers = attr_spec
            .split(",")
            .filter_map(|s| {
                let ch = s.trim();
                match ch {
                    s if s.starts_with("max_length") => {
                        max_length = Some(get_val(s));
                        None
                    }
                    s if s.starts_with("min_length") => {
                        min_length = Some(get_val(s));
                        None
                    }
                    s if s.starts_with("min") && valid_const(s) => {
                        min = Some(get_val(s));
                        None
                    }
                    s if s.starts_with("max") && valid_const(s) => {
                        max = Some(get_val(s));
                        None
                    }
                    s if s.starts_with("optional") => {
                        optional = true;
                        None
                    }
                    s if s.starts_with("monetary_cents") => {
                        monetary_cents = true;
                        None
                    }
                    s if s.starts_with("default") => {
                        default_value = Some(get_str_val(s));
                        None
                    }
                    s if !valid_const(s) => {
                        // eprintln!("{} is not a valid constraint", s);
                        None
                    }
                    "required" => None,
                    _ => Some(ch.to_string()),
                }
            })
            .collect::<Vec<_>>();

        Ok((
            FieldValidation {
                max_length,
                min_length,
                min,
                max,
                optional,
                monetary_cents,
                default_value,
            },
            leftovers,
        ))
    }

    pub(crate) fn generate_annotations(&self, is_model_or_enum: bool) -> TokenStream {
        let annotations: Vec<_> = [
            self.min_length.map(|r| {
                let r = LitInt::new(&r.to_string(), Span::call_site());
                quote!(#[validate(min_length = #r)])
            }),
            self.max_length.map(|r| {
                let r = LitInt::new(&r.to_string(), Span::call_site());
                quote!(#[validate(max_length = #r)])
            }),
            self.min.map(|r| {
                let r = LitInt::new(&r.to_string(), Span::call_site());
                quote!(#[validate(minimum = #r)])
            }),
            self.max.map(|r| {
                let r = LitInt::new(&r.to_string(), Span::call_site());
                quote!(#[validate(maximum = #r)])
            }),
        ]
        .iter()
        .filter_map(|v| v.clone())
        .collect();

        let submodel_annotation = if is_model_or_enum {
            quote!(#[validate])
        } else {
            quote!()
        };

        quote! {
            #(#annotations)*
            #submodel_annotation
        }
    }
}
