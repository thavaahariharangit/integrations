/*
 * Copyright (C) 2016-2025 Yuriy Yarosh
 * All rights reserved.
 *
 * SPDX-License-Identifier: MPL-2.0
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

mod enums;
mod structs;

pub use enums::*;
pub use structs::*;

use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use anyhow::*;

use bincode::{Decode, Encode};

#[derive(Debug, Clone, Copy, PartialEq, Encode, Decode)]
pub enum APIMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
    TRACE,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
pub struct APIEndpoint {
    pub name: String,
    pub path: String,
    pub method: APIMethod,

    pub mutating: bool,

    pub structs: Vec<APIStruct>,
    pub enums: Vec<APIEnum>,

    pub request_body: Option<APIStruct>,
    pub response_body: Option<APIStruct>,
    pub error_response_body: Option<APIStruct>,
}

impl APIEndpoint {
    pub fn new(
        name: String,
        path: String,
        method: APIMethod,
        mutating: bool,
        structs: Vec<APIStruct>,
        enums: Vec<APIEnum>,
        request_body: Option<APIStruct>,
        response_body: Option<APIStruct>,
        error_response_body: Option<APIStruct>,
    ) -> Self {
        Self {
            name,
            path,
            method,
            mutating,
            structs,
            enums,
            request_body,
            response_body,
            error_response_body,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
pub struct API {
    pub name: String,
    pub endpoints: Vec<APIEndpoint>,
}

impl API {
    pub fn new(name: String, endpoints: Vec<APIEndpoint>) -> Self {
        Self { name, endpoints }
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let mut file = File::create(path)?;
        let encoded: Vec<u8> = bincode::encode_to_vec(self, bincode::config::standard())?;
        file.write_all(&encoded)?;
        Ok(())
    }

    pub fn load(path: &Path) -> Result<Self> {
        let mut file = File::open(path)?;
        let mut encoded = Vec::new();
        file.read_to_end(&mut encoded)?;
        let decoded: (Self, usize) =
            bincode::decode_from_slice(&encoded, bincode::config::standard())?;
        Ok(decoded.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn create_test_api() -> API {
        let user_struct = APIStruct::new(
            "User".to_string(),
            vec![
                APIStructField::new("id".to_string(), FieldType::Number, None),
                APIStructField::new("name".to_string(), FieldType::String, None),
            ],
        );

        let error_struct = APIStruct::new(
            "Error".to_string(),
            vec![APIStructField::new(
                "message".to_string(),
                FieldType::String,
                None,
            )],
        );

        let endpoint1 = APIEndpoint {
            name: "GetUser".to_string(),
            path: "/users/{id}".to_string(),
            method: APIMethod::GET,
            mutating: false,
            structs: vec![user_struct.clone()],
            enums: vec![],
            request_body: None,
            response_body: Some(user_struct.clone()),
            error_response_body: Some(error_struct.clone()),
        };

        let endpoint2 = APIEndpoint {
            name: "CreateUser".to_string(),
            path: "/users".to_string(),
            method: APIMethod::POST,
            mutating: true,
            structs: vec![user_struct.clone()],
            enums: vec![],
            request_body: Some(user_struct.clone()),
            response_body: Some(user_struct.clone()),
            error_response_body: Some(error_struct.clone()),
        };

        API::new("MyTestAPI".to_string(), vec![endpoint1, endpoint2])
    }

    #[test]
    fn test_api_save_and_load() {
        let api = create_test_api();

        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_api.bin");

        api.save(&file_path).unwrap();

        let loaded_api = API::load(&file_path).unwrap();

        assert_eq!(api, loaded_api);
    }
}
