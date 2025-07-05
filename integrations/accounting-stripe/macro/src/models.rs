/*
    Copyright (C) 2017-2025 Yuriy Yarosh.
    All rights reserved.
*/

use bitsnap_integration::Client as HttpsClient;
use proc_macro2::*;
use quote::*;
use std::fs;
use syn::*;

mod codegen;
mod fields;

#[derive(Default, Debug, Clone)]
pub(crate) struct Model {
    pub name: String,
    pub fields: Vec<fields::Field>,
}

impl Model {
    fn scrape(html: String, name: &str) -> codegen::Result<Model> {
        return Ok(Model {
            name: name.to_string(),
            fields: vec![], //fields::Field::new(&html, &name),
        });
    }

    pub(crate) fn generate(&self) -> TokenStream {
        let expanded = quote! {};

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

pub(crate) fn generate(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let Input { name, url } = parse_macro_input!(input as Input);
    let model = scrape_models(&name.value(), &url.value());

    // let expanded = model.unwrap().generate();

    // expanded.into()
    quote! {}.into()
}

pub(crate) fn scrape_models(name: &str, url: &str) -> codegen::Result<Model> {
    let html = fs::read_to_string(HttpsClient::scrape_file_path(&url, "stripe"))
        .expect("Failed to read HTML file");
    Model::scrape(html, name)
}
