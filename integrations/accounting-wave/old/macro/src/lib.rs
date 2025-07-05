/*
    Copyright (C) 2017-2025 Yuriy Yarosh.
    All rights reserved.
*/

use proc_macro::TokenStream;

pub(crate) mod models;

#[proc_macro]
pub fn generate_model(input: TokenStream) -> TokenStream {
    models::generate(input)
}
