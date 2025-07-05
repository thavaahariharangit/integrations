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
        "https://developer.paypal.com/docs/api/tracking/v1/",
        "https://developer.paypal.com/docs/api/catalog-products/v1/",
        "https://developer.paypal.com/docs/api/customer-disputes/v1/",
        "https://developer.paypal.com/docs/api/identity/v1/",
        "https://developer.paypal.com/docs/api/identity/v2/",

        "https://developer.paypal.com/docs/api/invoicing/v2/",
        "https://developer.paypal.com/docs/api/orders/v2/",
        "https://developer.paypal.com/docs/api/partner-referrals/v2/",
        "https://developer.paypal.com/docs/api/payment-experience/v1/",
        "https://developer.paypal.com/docs/api/payment-tokens/v3/",
        "https://developer.paypal.com/docs/api/payments/v2/",
        "https://developer.paypal.com/docs/api/payments.payouts-batch/v1/",
        "https://developer.paypal.com/docs/api/referenced-payouts/v1/",
        "https://developer.paypal.com/docs/api/subscriptions/v1/",
        "https://developer.paypal.com/docs/api/transaction-search/v1/",
        "https://developer.paypal.com/docs/api/webhooks/v1/",
    ]).map(|url| {
        let client = &client;
        async move {
            println!("{}", url);
            let root = &HttpsClient::scrape_file_path(url, "paypal");
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
