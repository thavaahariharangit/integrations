/*
    Copyright (C) 2017-2025 Yuriy Yarosh.
    All rights reserved.
*/

use proc_macro::TokenStream;
use quote::quote;
use syn::*;

struct DeleteInput {
    name: LitStr,
    endpoint: LitStr,
}

impl parse::Parse for DeleteInput {
    fn parse(input: parse::ParseStream) -> Result<DeleteInput> {
        let name: LitStr = input.parse()?;
        input.parse::<Token![,]>()?; // Expect a comma
        let endpoint: LitStr = input.parse()?;
        Ok(DeleteInput { name, endpoint })
    }
}

pub(crate) fn generate(input: TokenStream) -> TokenStream {
    let DeleteInput { name, endpoint } = parse_macro_input!(input as DeleteInput);

    let name_ident = Ident::new(&name.value(), name.span());

    let expanded = quote! {
        pub struct #name_ident {

        }
    };

    eprintln!("Generated model: {}", expanded);

    expanded.into()
}
