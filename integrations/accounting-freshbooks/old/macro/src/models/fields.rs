/*
    Copyright (C) 2017-2025 Yuriy Yarosh.
    All rights reserved.
*/
use crate::models::*;
use proc_macro2::TokenStream;
use quote::*;
use scraper::*;
use std::collections::*;
use std::hash::{Hash, Hasher};

/// Represents the parsed model fields from the ChargeBee API documentation
/// Used to generate appropriate Rust type definitions with validation constraints
#[derive(Default, Debug, Clone)]
pub(crate) struct Field {
    pub(crate) field_name: String,
    pub(crate) field_type: String,

    pub is_list: bool,
    pub is_optional: bool,

    pub submodel: Option<Model>,
}

impl PartialEq for Field {
    fn eq(&self, other: &Self) -> bool {
        self.field_name == other.field_name // && self.field_type == other.field_type
    }
}

impl Eq for Field {}
impl Hash for Field {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(self.field_name.clone().to_string().as_bytes()) // + ":" + &self.field_type).as_bytes());
    }

    fn hash_slice<H: Hasher>(data: &[Self], state: &mut H)
    where
        Self: Sized,
    {
        let mut entries = data.to_vec();
        entries.dedup();

        let joined = entries
            .iter()
            .map(|v| v.field_name.clone().to_string() + ":" + &v.field_type)
            .collect::<Vec<String>>()
            .join(",");
        state.write(joined.as_bytes());
    }
}

impl Field {
    pub(crate) fn new(group: &ElementRef, model_name: &str) -> HashSet<Field> {
        let opt_selector = Selector::parse("td u").unwrap();

        let fields = group
            .children()
            .filter_map(|t| {
                let c = t.children().collect::<Vec<_>>();
                if c.len() == 3 {
                    let name_elem = ElementRef::wrap(*c.get(0).unwrap()).unwrap();
                    let mut name = name_elem
                        .text()
                        .collect::<String>()
                        .trim()
                        .to_lowercase()
                        .to_string();
                    name = name
                        .replace(" ", "")
                        .replace("\t", "")
                        .replace("\n", "")
                        .replace("*", "");

                    let is_opt = name_elem.select(&opt_selector).next().is_none();

                    let t = ElementRef::wrap(*c.get(1).unwrap())
                        .unwrap()
                        .text()
                        .collect::<String>()
                        .trim()
                        .to_lowercase()
                        .to_string();
                    if t != "type" && name != "field" {
                        let is_field = !name.trim().chars().nth(0).unwrap().is_alphabetic();
                        if is_field {
                            name = name.chars().skip(1).collect();
                        }

                        let is_model: bool = t.contains("object");

                        Some((name, t, is_model, is_field, is_opt))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        let obj_idx = fields
            .iter()
            .enumerate()
            .into_iter()
            .filter_map(|(i, (_, _, is_model, _, _))| if *is_model { Some(i) } else { None })
            .collect::<Vec<usize>>();

        let mut models = obj_idx
            .iter()
            .map(|i| {
                let name = fields.get(*i).unwrap().0.clone();

                let last_field_idx = fields
                    .iter()
                    .skip(*i + 1)
                    .enumerate()
                    .find(|(_, (_, _, _, is_field, _))| !is_field)
                    .map(|(idx, _)| idx + *i + 1)
                    .unwrap_or(fields.len());

                let model_fields = fields[*i + 1..last_field_idx]
                    .iter()
                    .map(|(field_name, field_type, _, _, is_opt)| {
                        Self::parse(field_name, &field_type, false, *is_opt, model_name, vec![])
                    })
                    .collect::<Vec<_>>();

                Model {
                    name: model_name.to_string() + &snake_case_to_camel_case(&name),
                    fields: model_fields.iter().map(|f| f.clone()).collect(),
                }
            })
            .collect::<Vec<_>>();
        models.dedup_by(|a, b| a.name == b.name);

        let mut fields = fields
            .iter()
            .filter(|(_, _, _, is_field, _)| !is_field)
            .map(|(field_name, field_type, is_model, _, is_opt)| {
                Self::parse(
                    field_name,
                    &field_type,
                    *is_model,
                    *is_opt,
                    model_name,
                    models.clone(),
                )
            })
            .collect::<Vec<_>>();

        fields.dedup_by(|a, b| a.field_name == b.field_name);
        fields.iter().map(|f| f.clone()).collect()
    }

    fn parse(
        field_name: &str,
        field_type: &str,
        is_model: bool,
        is_opt: bool,
        model_name: &str,
        models: Vec<Model>,
    ) -> Field {
        let mut field_type = field_type.to_string();
        let is_optional = field_type.contains("(optional)") || is_opt;
        if is_optional {
            field_type = field_type.replace("(optional)", "").trim().to_string();
        }

        let mut is_list = field_type.contains("array<");
        if is_list {
            field_type = field_type
                .replace("array<", "")
                .replace(">", "")
                .trim()
                .to_string();
        }

        field_type = match field_type {
            f if f == "integer" => "i64".to_string(),
            f if f == "int" => "i64".to_string(),
            f if f == "it" => "i64".to_string(), // int typo for services
            f if f == "float" => "f64".to_string(),
            f if f == "double" => "f64".to_string(),
            f if f == "decimal" => "BigDecimal".to_string(),
            f if f == "boolean" => "bool".to_string(),
            f if f == "bool" => "bool".to_string(),
            f if f == "string" => "String".to_string(),
            f if f == "date" => "String".to_string(), // YYYY-MM-DD
            f if f == "datetime" => "String".to_string(), // YYYY-MM-DD HH:MM:SS
            f if f == "object" => "".to_string(),
            f if f.contains("array") => {
                is_list = true;
                "String".to_string()
            }
            _ => {
                eprintln!("unknown field type: {}", field_type);
                "String".to_string()
            }
        };

        let mut submodel = None;
        if is_model {
            submodel = models
                .iter()
                .find(|m| m.name == Self::submodel_name(field_name, model_name))
                .map(|m| m.clone());
            field_type = submodel
                .clone()
                .map(|m| m.name)
                .unwrap_or_else(|| Self::submodel_name(field_name, model_name));
        }

        common::replace_common_model(
            Field {
                field_name: field_name.to_string(),
                field_type,
                is_list,
                is_optional,
                submodel,
            },
            model_name,
        )
    }

    fn submodel_name(name: &str, model_name: &str) -> String {
        model_name.to_string() + &snake_case_to_camel_case(&name)
    }

    pub(crate) fn generate(&self) -> TokenStream {
        let name_ident = name_ident(&self.field_name);
        let type_ident = type_ident(&self.field_type, self.is_optional, self.is_list);

        self.generate_submodel_file();

        quote! {
            pub #name_ident: #type_ident
        }
    }

    pub(crate) fn generate_submodel_file(&self) {
        if let Some(submodel) = self.submodel.clone() {
            if submodel.fields.is_empty() {
                generate_submodel_file_stub("freshbooks", &submodel.name);
            }
        }
    }
}
