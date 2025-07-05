/*
    Copyright (C) 2017-2025 Yuriy Yarosh.
    All rights reserved.
*/

use proc_macro::TokenStream;
use quote::quote;
use syn::{Ident, LitStr, Token, parse, parse_macro_input};

struct GenerateListOfInput {
    plural: LitStr,
    name: LitStr,
    endpoint: LitStr,
    sub_endpoint: LitStr,
}

// Implement syn::Parse for the struct
impl parse::Parse for GenerateListOfInput {
    fn parse(input: parse::ParseStream) -> syn::Result<GenerateListOfInput> {
        let plural: LitStr = input.parse()?;
        input.parse::<Token![,]>()?; // Expect a comma
        let name: LitStr = input.parse()?;
        input.parse::<Token![,]>()?; // Expect a comma
        let endpoint: LitStr = input.parse()?;
        input.parse::<Token![,]>()?; // Expect a comma
        let sub_endpoint: LitStr = input.parse()?;

        Ok(GenerateListOfInput {
            plural,
            name,
            endpoint,
            sub_endpoint,
        })
    }
}

pub(crate) fn generate(input: TokenStream) -> TokenStream {
    let GenerateListOfInput {
        plural,
        name,
        endpoint,
        sub_endpoint,
    } = parse_macro_input!(input as GenerateListOfInput);

    let name_ident = Ident::new(&name.value(), name.span());
    let endpoint_str = endpoint.value();
    let method_name = Ident::new(
        &format!("list_{}", plural.value().to_lowercase()),
        plural.span(),
    );

    let expanded = quote! {
       pub async fn #method_name(&self) -> Result<Vec<#name_ident>, std::io::Error> {
            let url = format!("{}/{}", self.base_url, #endpoint_str);
            let content = self.client.get(&url);
            // let response = client.get(&url).send().await?;
            // let items = response.json::<Vec<#name_ident>>().await?;
            Ok(Vec::new())
        }
    };

    // eprintln!("Generated code: {}", expanded);

    expanded.into()
}
