/*
    Copyright (C) 2017-2025 Yuriy Yarosh.
    All rights reserved.
*/

use proc_macro::TokenStream;
use quote::quote;
use syn::*;

struct CreateInput {
    name: LitStr,
    endpoint: LitStr,
}

impl parse::Parse for CreateInput {
    fn parse(input: parse::ParseStream) -> Result<CreateInput> {
        let name: LitStr = input.parse()?;
        input.parse::<Token![,]>()?; // Expect a comma
        let endpoint: LitStr = input.parse()?;
        Ok(CreateInput { name, endpoint })
    }
}

pub(crate) fn generate(input: TokenStream) -> TokenStream {
    let CreateInput { name, endpoint } = parse_macro_input!(input as CreateInput);

    let name_ident = Ident::new(&name.value(), name.span());

    let expanded = quote! {
        pub struct #name_ident {

        }
    };

    eprintln!("Generated model: {}", expanded);

    expanded.into()
}
