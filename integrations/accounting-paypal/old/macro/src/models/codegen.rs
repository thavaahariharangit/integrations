/*
    Copyright (C) 2017-2025 Yuriy Yarosh.
    All rights reserved.
*/

#[derive(thiserror::Error, Debug)]
pub(crate) enum CodegenError {
    // #[error("Expected only type after parsing validation values: {unknown_attrs:?}")]
    // ExpectedOnlyType { unknown_attrs: Vec<String> },
}

pub(crate) type Result<T> = std::result::Result<T, CodegenError>;
