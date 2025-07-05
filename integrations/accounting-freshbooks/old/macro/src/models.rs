/*
    Copyright (C) 2017-2025 Yuriy Yarosh.
    All rights reserved.
*/

use bitsnap_codegen_util::*;
use bitsnap_integration::Client as HttpsClient;
use std::collections::HashSet;

use proc_macro2::{Ident, Span, TokenStream};
use quote::*;
use scraper::*;
use std::fs;
use syn::{LitStr, Token, parse, parse_macro_input};

pub(crate) mod codegen;
mod fields;
use fields::*;
mod common;

#[derive(Default, Debug, Clone)]
pub(crate) struct Model {
    pub name: String,
    pub fields: HashSet<Field>,
}

impl PartialEq for Model {
    fn eq(&self, other: &Self) -> bool {
        self.fields == other.fields
    }
}

impl Model {
    const SCRAPE_PAGE_HEADER_SELECTOR: &'static str = "div.col-content h2";

    const SCRAPE_FIELDS_TABLE_SELECTOR: &'static str = "table tbody";

    fn scrape(html: String, name: &str) -> codegen::Result<Model> {
        let document = Html::parse_document(&html);

        let header_selector = Selector::parse(Self::SCRAPE_PAGE_HEADER_SELECTOR).unwrap();
        let fields_table_selector = Selector::parse(Self::SCRAPE_FIELDS_TABLE_SELECTOR).unwrap();

        let header_elem = document
            .select(&header_selector)
            .find(|h| h.text().collect::<String>().trim() == "Field Descriptions")
            .expect(&format!("fields description {} not found", name));

        let doc = header_elem.ancestors().nth(0).unwrap();
        let (pos, _) = doc
            .children()
            .enumerate()
            .find(|(_, c)| {
                ElementRef::wrap(*c)
                    .map(|c| c.text().collect::<String>() == header_elem.text().collect::<String>())
                    .unwrap_or(false)
            })
            .unwrap();
        let fields_table_node = doc
            .children()
            .skip(pos + 1)
            .find(|c| {
                c.value()
                    .as_element()
                    .map(|e| e.name() == "figure")
                    .unwrap_or(false)
            })
            .unwrap();
        let fields_table = ElementRef::wrap(fields_table_node)
            .unwrap()
            .select(&fields_table_selector)
            .nth(0)
            .unwrap();

        Ok(Model {
            name: name.to_string(),
            fields: Field::new(&fields_table, &name),
        })
    }

    pub(crate) fn generate(&self) -> TokenStream {
        let mut submodels = self
            .fields
            .iter()
            .filter_map(|f| f.submodel.clone())
            .collect::<Vec<_>>();
        let modules = submodels
            .iter()
            .filter(|s| s.fields.is_empty())
            .map(|s| s.name.clone())
            .collect::<Vec<_>>();
        submodels = submodels
            .iter()
            .filter(|s| !s.fields.is_empty())
            .map(|m| m.clone())
            .collect::<Vec<_>>();
        submodels.dedup_by(|a, b| a.name == b.name);

        let common = common::get();
        submodels = submodels
            .iter()
            .filter(|s| !common.iter().any(|c| c.name == s.name))
            .map(|s| s.clone())
            .collect();

        let submodels = submodels.iter().map(|s| s.generate()).collect::<Vec<_>>();
        let fields = self.fields.iter().map(|f| f.generate()).collect::<Vec<_>>();

        let name_ident = Ident::new(&self.name, Span::call_site());

        let modules: Vec<_> = modules
            .iter()
            .map(|name| {
                let module_name = Ident::new(&(camel_case_to_snake_case(&name)), Span::call_site());

                quote! {
                    mod #module_name;
                    use #module_name::*;
                }
            })
            .collect();

        let expanded = quote! {
            #(#modules)*

            #(#submodels)*

            #[derive(Default, Debug, Clone, Serialize, Deserialize)]
            #[serde(rename_all = "snake_case")]
            pub struct #name_ident {
                #(#fields),*
            }
        };

        // eprintln!("{}", expanded);

        expanded.into()
    }
}

/// Macro Input
#[derive(Debug)]
struct Input {
    name: LitStr,
    url: LitStr,
}

impl parse::Parse for Input {
    fn parse(input: parse::ParseStream) -> syn::Result<Input> {
        let name: LitStr = input.parse()?;
        input.parse::<Token![,]>()?;
        let url: LitStr = input.parse()?;
        Ok(Input { name, url })
    }
}

pub(crate) fn generate_common_models() -> proc_macro::TokenStream {
    let models = common::get()
        .iter()
        .map(|m| m.generate())
        .collect::<Vec<_>>();

    (quote! {
        #(#models)*
    })
    .into()
}

pub(crate) fn generate(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let Input { name, url } = parse_macro_input!(input as Input);
    let model = scrape_models(&name.value(), &url.value());

    let expanded = model.unwrap().generate();

    expanded.into()
}

pub(crate) fn scrape_models(name: &str, url: &str) -> codegen::Result<Model> {
    let html = fs::read_to_string(HttpsClient::scrape_file_path(&url, "freshbooks"))
        .expect("Failed to read HTML file");
    Model::scrape(html, name)
}
