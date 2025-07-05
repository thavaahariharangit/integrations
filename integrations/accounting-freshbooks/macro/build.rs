/*
    Copyright (C) 2017-2025 Yuriy Yarosh.
    All rights reserved.
*/

// use bitsnap_integration::Client as HttpsClient;
use futures::*;
use std::fs;
use std::path::Path;

const PARALLEL_FETCHES: usize = 8;
#[tokio::main]
async fn main() {
    // let client = HttpsClient::new();
    // stream::iter(vec![
    //     "https://www.freshbooks.com/api/bill_payments",
    //     "https://www.freshbooks.com/api/vendors",
    //     "https://www.freshbooks.com/api/bills",
    //     "https://www.freshbooks.com/api/clients",
    //     "https://www.freshbooks.com/api/credits",
    //     "https://www.freshbooks.com/api/estimates",
    //     "https://www.freshbooks.com/api/expenses",
    //     "https://www.freshbooks.com/api/invoice_profiles",
    //     "https://www.freshbooks.com/api/invoices",
    //     "https://www.freshbooks.com/api/items",
    //     "https://www.freshbooks.com/api/other_income",
    //     "https://www.freshbooks.com/api/payments",
    //     "https://www.freshbooks.com/api/project",
    //     "https://www.freshbooks.com/api/services",
    //     "https://www.freshbooks.com/api/tasks",
    //     "https://www.freshbooks.com/api/taxes",
    //     "https://www.freshbooks.com/api/team-members",
    //     "https://www.freshbooks.com/api/time_entries",
    // ])
    // .map(|url| {
    //     let client = &client;
    //     async move {
    //         println!("{}", url);
    //         let root = &HttpsClient::scrape_file_path(url, "freshbooks");
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
