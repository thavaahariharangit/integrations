/*
    Copyright (C) 2017-2025 Yuriy Yarosh.
    All rights reserved.
*/
use crate::models::*;

use proc_macro2::*;
use quote::*;
use scraper::*;
use syn::*;

use bitsnap_codegen_util::*;

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

impl Field {
    pub(crate) fn new(group: &ElementRef, model_name: &str) -> Vec<Field> {
        vec![]
    }

    fn parse(
        field_name: &str,
        field_type: &str,
        is_model: bool,
        is_opt: bool,
        model_name: &str,
        models: Vec<Model>,
    ) -> Field {
        Field::default()
    }

    fn submodel_name(name: &str, model_name: &str) -> String {
        model_name.to_string() + &snake_case_to_camel_case(&name)
    }

    pub(crate) fn generate(&self) -> TokenStream {
        // let name_ident = name_ident(&self.field_name);
        // let type_ident = type_ident(&self.field_type, self.is_optional, self.is_list);

        // self.generate_submodel_file();

        quote! {}
    }

    pub(crate) fn generate_submodel_file(&self) {
        if let Some(submodel) = self.submodel.clone() {
            if submodel.fields.is_empty() {
                generate_submodel_file_stub("myob", &submodel.name);
            }
        }
    }
}
