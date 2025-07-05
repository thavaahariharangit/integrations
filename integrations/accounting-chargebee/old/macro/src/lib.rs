/*
    Copyright (C) 2017-2025 Yuriy Yarosh.
    All rights reserved.
*/

use proc_macro::TokenStream;

mod create;
mod delete;
mod list;
mod list_of;
pub(crate) mod models;
mod retrieve;
mod update;
#[proc_macro]
pub fn generate_list(input: TokenStream) -> TokenStream {
    list::generate(input)
}

#[proc_macro]
pub fn generate_list_of(input: TokenStream) -> TokenStream {
    list_of::generate(input)
}

#[proc_macro]
pub fn generate_retrieve(input: TokenStream) -> TokenStream {
    retrieve::generate(input)
}

#[proc_macro]
pub fn generate_create(input: TokenStream) -> TokenStream {
    create::generate(input)
}

#[proc_macro]
pub fn generate_update(input: TokenStream) -> TokenStream {
    update::generate(input)
}

#[proc_macro]
pub fn generate_delete(input: TokenStream) -> TokenStream {
    delete::generate(input)
}

#[proc_macro]
pub fn generate_model(input: TokenStream) -> TokenStream {
    models::generate(input)
}

#[proc_macro]
pub fn generate_enums_rs(_: TokenStream) -> TokenStream {
    models::generate_enums_rs()
}
