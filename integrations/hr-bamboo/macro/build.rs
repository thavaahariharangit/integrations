/*
    Copyright (C) 2017-2025 Yuriy Yarosh.
    All rights reserved.
*/

use bitsnap_integration::Client as HttpsClient;
use futures::*;
use std::fs;
use std::path::Path;

const PARALLEL_FETCHES: usize = 8;
#[tokio::main]
async fn main() {
    // let client = HttpsClient::new();
    // stream::iter(vec![

    // ]).map(|url| {
    //     let client = &client;
    //     async move {
    //         println!("{}", url);
    //         let root = &HttpsClient::scrape_file_path(url, "bamboo");
    //         let output_path = Path::new(root);
    //         if !output_path.exists() {
    //             eprintln!("Downloading {:?}", url);
    //             let response = client.get(url).await.expect("Failed to fetch URL");
    //             let response_str = String::from_utf8(response.to_vec())
    //                 .expect("Failed to convert response to string");
    //             fs::write(output_path, response_str).expect("Failed to write response to file");
    //         }
    //     }
    // })
    // .buffer_unordered(PARALLEL_FETCHES)
    // .for_each(|_| async {})
    // .await;
}
