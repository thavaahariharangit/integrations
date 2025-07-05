/*
    Copyright (C) 2017-2025 Yuriy Yarosh.
    All rights reserved.
*/

use proc_macro::TokenStream;
use quote::quote;
use syn::{Ident, LitStr, Token, parse, parse_macro_input};

struct RetrieveInput {
    name: LitStr,
    endpoint: LitStr,
}

// Implement syn::Parse for the struct
impl parse::Parse for RetrieveInput {
    fn parse(input: parse::ParseStream) -> syn::Result<RetrieveInput> {
        let name: LitStr = input.parse()?;
        input.parse::<Token![,]>()?; // Expect a comma
        let endpoint: LitStr = input.parse()?;
        Ok(RetrieveInput { name, endpoint })
    }
}

pub(crate) fn generate(input: TokenStream) -> TokenStream {
    let RetrieveInput { name, endpoint } = parse_macro_input!(input as RetrieveInput);

    let name_ident = Ident::new(&name.value(), name.span());
    let endpoint_str = endpoint.value();
    let method_name = Ident::new(
        &format!("list_{}", name.value().to_lowercase()),
        endpoint.span(),
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
