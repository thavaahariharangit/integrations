/*
    Copyright (C) 2017-2025 Yuriy Yarosh.
    All rights reserved.
*/
use bitsnap_codegen_util::*;

use proc_macro2::{Span, TokenStream};
use quote::*;
use scraper::selectable::Selectable;
use scraper::selector::CssLocalName;
use scraper::*;
use std::collections::*;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use syn::*;

use crate::models::validation::*;
use crate::models::*;

/// Represents the parsed model fields from the ChargeBee API documentation
/// Used to generate appropriate Rust type definitions with validation constraints
#[derive(Default, Debug, Clone)]
pub(crate) struct Field {
    pub validation: FieldValidation,

    field_name: String,
    field_type: String,

    pub is_list: bool,
    pub is_enum: bool,
    pub is_model: bool,

    pub submodel: Option<String>,
    pub enum_variants: Vec<String>,
}

impl PartialEq for Field {
    fn eq(&self, other: &Self) -> bool {
        self.field_name == other.field_name && self.field_type == other.field_type
    }
}
impl Field {
    const NEW_FIELD_NAME_SELECTOR: &'static str = "div div.cb-list-item > samp";
    const NEW_FIELD_ATTR_SELECTOR: &'static str = "div div.cb-list-desc > dfn";
    const NEW_ENUM_VARIANTS_SELECTOR: &'static str =
        "div div.cb-list-desc div div.cb-enum-parent samp";

    pub(crate) fn new(group: &ElementRef, model_name: &str) -> Vec<Field> {
        let name_selector = Selector::parse(Self::NEW_FIELD_NAME_SELECTOR).unwrap();
        let attr_selector = Selector::parse(Self::NEW_FIELD_ATTR_SELECTOR).unwrap();
        let enum_variant_selector = Selector::parse(Self::NEW_ENUM_VARIANTS_SELECTOR).unwrap();

        group
            .child_elements()
            .filter(|el| {
                el.has_class(
                    &CssLocalName::from("cb-list"),
                    CaseSensitivity::AsciiCaseInsensitive,
                )
            })
            .map(|el| {
                let name = el
                    .select(&name_selector)
                    .next()
                    .unwrap()
                    .text()
                    .collect::<String>()
                    .trim()
                    .to_string();
                let attr_spec = el
                    .select(&attr_selector)
                    .next()
                    .unwrap()
                    .text()
                    .collect::<String>()
                    .trim()
                    .to_string();
                let enum_variants: Vec<_> = el
                    .select(&enum_variant_selector)
                    .map(|el| el.text().collect::<String>().trim().to_string())
                    .collect();

                Self::parse(model_name, &name, &attr_spec, enum_variants)
                    .expect(format!("Failed to parse '{}.{}'", model_name, name).as_str())
            })
            .collect::<Vec<_>>()
    }

    pub(crate) fn normalize_submodel_name(field_name: &str, parent: &str) -> String {
        let camel_cased = snake_case_to_camel_case(field_name);

        if camel_cased.starts_with(parent) {
            camel_cased.to_string()
        } else {
            format!("{}{}", parent, camel_cased)
        }
    }

    fn parse(
        model_name: &str,
        field_name: &str,
        attr_spec: &str,
        enum_variants: Vec<String>,
    ) -> codegen::Result<Field> {
        let (validation, rest) = FieldValidation::parse(attr_spec)?;

        if rest.len() != 1 {
            return Err(codegen::CodegenError::ExpectedOnlyType {
                unknown_attrs: rest,
            });
        }

        let mut is_list = false;

        let mut t = rest.first().unwrap().as_str();
        if t.starts_with("list of ") {
            t = &t["list of ".len()..].trim();
            is_list = true;
        }

        let is_enum = enum_variants.len() > 0 || t.starts_with("enumerated");
        let mut is_model = false;
        let mut submodel = None;

        let matched_type = HashMap::from([
            ("integer", "i64"),
            ("long", "i64"),
            ("float", "f64"),
            ("double", "f64"),
            ("boolean", "bool"),
            ("string", "String"),
            ("date", "chrono::NaiveDate"),
            ("time", "chrono::NaiveTime"),
            ("in cents", "i64"),
            ("bigdecimal", "BigDecimal"),
            ("enumerated string", "String"),
            ("timestamp(UTC) in seconds", "u64"),
            ("strings", "String"),
            ("jsonarray", "Vec<serde_json::Value>"),
            ("jsonobject", "serde_json::Value"),
        ])
        .iter()
        .find_map(|(r, v)| if *r == t { Some(v.to_string()) } else { None });

        let normalized_name =
            Self::normalize_submodel_name(if is_enum { field_name } else { t }, model_name);
        let mut field_type = matched_type.unwrap_or_else(|| {
            is_model = true;
            submodel = Some(normalized_name.clone());
            normalized_name.clone()
        });

        if is_enum {
            submodel = Some(normalized_name.clone());
            field_type = normalized_name;
        }

        let validation = if (validation.min.is_some() || validation.max.is_some())
            && (field_type == "String" || is_list)
        {
            validation.swap_ranges()
        } else {
            validation
        };

        Ok(Field {
            validation,
            field_name: field_name.to_string(),
            field_type,
            is_list,
            is_enum,
            is_model,
            submodel,
            enum_variants,
        })
    }

    pub(crate) fn generate(&self, model_name: &str) -> TokenStream {
        let default_annotation = if Self::skipped_default_value(
            self.validation
                .default_value
                .as_ref()
                .unwrap_or(&"".to_string()),
        ) {
            quote!()
        } else {
            self.validation
                .default_value
                .clone()
                .map(|_| {
                    let default_value_func = LitStr::new(
                        &(model_name.to_string() + "::default_" + &self.field_name),
                        Span::call_site(),
                    );
                    quote!(#[serde(default = #default_value_func)])
                })
                .unwrap_or_else(|| quote!())
        };

        let name_ident = name_ident(&self.field_name);
        let type_ident = type_ident(&self.field_type, self.validation.optional, self.is_list);

        let validation_annotations = self
            .validation
            .generate_annotations(self.is_model || self.is_enum);

        quote! {
            #validation_annotations
            #default_annotation
            pub #name_ident: #type_ident
        }
    }

    fn skipped_default_value(s: &str) -> bool {
        let skip_values: Vec<_> = vec!["0", "0.0", "false", "iso6523-actorid-upis"];
        skip_values.iter().find(|e| **e == s).is_some()
    }

    pub(crate) fn generate_default_func(&self) -> TokenStream {
        if Self::skipped_default_value(
            self.validation
                .default_value
                .as_ref()
                .unwrap_or(&"".to_string()),
        ) {
            return TokenStream::default();
        }

        self.validation
            .default_value
            .clone()
            .map(|v| {
                let value = match (self.validation.optional, self.field_type.as_str()) {
                    (true, "bool") => {
                        let v = LitBool::new(v == "true", Span::call_site());
                        quote!(Some(#v))
                    }
                    (true, "i64") => {
                        let v = LitInt::new(&*v, Span::call_site());
                        quote!(Some(#v))
                    }
                    (true, "u64") => {
                        let v = LitInt::new(&*v, Span::call_site());
                        quote!(Some(#v))
                    }
                    (true, "f64") => {
                        let v = LitInt::new(&*v, Span::call_site());
                        quote!(Some(#v))
                    }
                    (true, "String") => {
                        let v = LitStr::new(&*v, Span::call_site());
                        quote!(Some(#v.to_string()))
                    }
                    (false, "bool") => {
                        let v = LitBool::new(v == "true", Span::call_site());
                        quote!(#v)
                    }
                    (false, "i64") => {
                        let v = LitInt::new(&*v, Span::call_site());
                        quote!(#v)
                    }
                    (false, "u64") => {
                        let v = LitInt::new(&*v, Span::call_site());
                        quote!(#v)
                    }
                    (false, "f64") => {
                        let v = LitInt::new(&*v, Span::call_site());
                        quote!(#v)
                    }
                    (false, "String") => {
                        let v = LitStr::new(&*v, Span::call_site());
                        quote!(#v.to_string())
                    }
                    (true, _) => {
                        if self.is_enum {
                            let v = Ident::new(&*snake_case_to_camel_case(&v), Span::call_site());
                            let t = Ident::new(&self.submodel.clone().unwrap(), Span::call_site());
                            quote!(Some(#t::#v))
                        } else {
                            let v = Ident::new(&*v, Span::call_site());
                            quote!(Some(#v))
                        }
                    }
                    _ => {
                        if self.is_enum {
                            let v = Ident::new(&*snake_case_to_camel_case(&v), Span::call_site());
                            let t = type_ident(
                                &self.field_type,
                                self.validation.optional,
                                self.is_list,
                            );
                            quote!(#t::#v)
                        } else {
                            let v = Ident::new(&*v, Span::call_site());
                            quote!(#v)
                        }
                    }
                };

                let fn_name_ident = Ident::new(
                    &("default_".to_string() + &self.field_name),
                    Span::call_site(),
                );
                let type_ident =
                    type_ident(&self.field_type, self.validation.optional, self.is_list);

                quote! {
                    fn #fn_name_ident() -> #type_ident {
                        #value
                    }
                }
            })
            .unwrap_or(TokenStream::default())
    }

    pub(crate) fn generate_enums_file(self) {
        if self.is_enum {
            generate_enums_file(
                "chargebee",
                &self.submodel.unwrap().clone(),
                self.enum_variants,
            );
        }
    }

    pub(crate) fn generate_submodel_file(self) {
        if self.is_model {
            generate_submodel_file_stub("chargebee", &self.submodel.unwrap().clone());
        }
    }
}
