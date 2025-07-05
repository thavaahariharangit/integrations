/*
    Copyright (C) 2017-2025 Yuriy Yarosh.
    All rights reserved.
*/

use std::fs;
use std::path::Path;


fn main() {
    // let client = HttpsClient::new();

    // let output_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../clients/accounting_wave/graphql/types.json".to_string());
    // if !output_path.exists() {
    //     eprintln!("Downloading {:?}", GRAPHQL_ENDPOINT);
    //     let response = client.post_json(GRAPHQL_ENDPOINT, "{\"query\":\"{ __schema { types { name } } }\"}").await.expect("Failed to fetch URL");

    //     eprintln!("{}", output_path.to_str().unwrap());

    //     let response_str = String::from_utf8(response.to_vec())
    //         .expect("Failed to convert response to string");
    //     fs::write(output_path, response_str).expect("Failed to write response to file");
    // }
}
