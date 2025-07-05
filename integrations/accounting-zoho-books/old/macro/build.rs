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
    let client = HttpsClient::new();
    stream::iter(vec![
        "https://www.zoho.com/books/api/v3/contacts",
        "https://www.zoho.com/books/api/v3/contact-persons",
        "https://www.zoho.com/books/api/v3/estimates",
        "https://www.zoho.com/books/api/v3/sales-order",
        "https://www.zoho.com/books/api/v3/invoices",
        "https://www.zoho.com/books/api/v3/recurring-invoices",
        "https://www.zoho.com/books/api/v3/credit-notes",
        "https://www.zoho.com/books/api/v3/customer-payments",
        "https://www.zoho.com/books/api/v3/expenses",
        "https://www.zoho.com/books/api/v3/recurring-expenses",
        "https://www.zoho.com/books/api/v3/retainer-invoices",
        "https://www.zoho.com/books/api/v3/purchase-order",
        "https://www.zoho.com/books/api/v3/bills",
        "https://www.zoho.com/books/api/v3/recurring-bills",
        "https://www.zoho.com/books/api/v3/vendor-credits",
        "https://www.zoho.com/books/api/v3/vendor-payments",
        "https://www.zoho.com/books/api/v3/custom-modules",
        "https://www.zoho.com/books/api/v3/bank-accounts",
        "https://www.zoho.com/books/api/v3/bank-transactions",
        "https://www.zoho.com/books/api/v3/bank-rules",
        "https://www.zoho.com/books/api/v3/chart-of-accounts",
        "https://www.zoho.com/books/api/v3/journals",
        "https://www.zoho.com/books/api/v3/base-currency-adjustment",
        "https://www.zoho.com/books/api/v3/projects",
        "https://www.zoho.com/books/api/v3/tasks",
        "https://www.zoho.com/books/api/v3/time-entries",
        "https://www.zoho.com/books/api/v3/users",
        "https://www.zoho.com/books/api/v3/items",
        "https://www.zoho.com/books/api/v3/currency",
        "https://www.zoho.com/books/api/v3/taxes",
        "https://www.zoho.com/books/api/v3/opening-balance",
    ]).map(|url| {
        let client = &client;
        async move {
            println!("{}", url);
            let root = &HttpsClient::scrape_file_path(url, "zoho_books");
            let output_path = Path::new(root);
            if !output_path.exists() {
                eprintln!("Downloading {:?}", url);
                let response = client.get(url).await.expect("Failed to fetch URL");
                let response_str = String::from_utf8(response.to_vec())
                    .expect("Failed to convert response to string");
                fs::write(output_path, response_str).expect("Failed to write response to file");
            }
        }
    })
    .buffer_unordered(PARALLEL_FETCHES)
    .for_each(|_| async {})
    .await;
}
