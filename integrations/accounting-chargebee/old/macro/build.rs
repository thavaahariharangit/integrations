/*
    Copyright (C) 2017-2025 Yuriy Yarosh.
    All rights reserved.
*/

use bitsnap_integration::Client as HttpsClient;
use futures::*;
use std::fs;
use std::path::Path;
use std::fs::create_dir_all;

const PARALLEL_FETCHES: usize = 8;
#[tokio::main]
async fn main() {
    let client = HttpsClient::new();
    create_dir_all("./.pages").unwrap();

    stream::iter(vec![
        "https://apidocs.chargebee.com/docs/api/advance_invoice_schedules",
        "https://apidocs.chargebee.com/docs/api/attached_items",
        "https://apidocs.chargebee.com/docs/api/business_entities",
        "https://apidocs.chargebee.com/docs/api/business_entity_transfers",
        "https://apidocs.chargebee.com/docs/api/contacts",
        "https://apidocs.chargebee.com/docs/api/contract_terms",
        "https://apidocs.chargebee.com/docs/api/coupon_codes",
        "https://apidocs.chargebee.com/docs/api/coupons",
        "https://apidocs.chargebee.com/docs/api/coupon_sets",
        "https://apidocs.chargebee.com/docs/api/credit_notes",
        "https://apidocs.chargebee.com/docs/api/currencies",
        "https://apidocs.chargebee.com/docs/api/customers",
        "https://apidocs.chargebee.com/docs/api/differential_prices",
        "https://apidocs.chargebee.com/docs/api/discounts",
        "https://apidocs.chargebee.com/docs/api/entitlements",
        "https://apidocs.chargebee.com/docs/api/events",
        "https://apidocs.chargebee.com/docs/api/features",
        "https://apidocs.chargebee.com/docs/api/gifts",
        "https://apidocs.chargebee.com/docs/api/hierarchies",
        "https://apidocs.chargebee.com/docs/api/impacted_items",
        "https://apidocs.chargebee.com/docs/api/impacted_subscriptions",
        "https://apidocs.chargebee.com/docs/api/invoices",
        "https://apidocs.chargebee.com/docs/api/item_families",
        "https://apidocs.chargebee.com/docs/api/item_prices",
        "https://apidocs.chargebee.com/docs/api/items",
        "https://apidocs.chargebee.com/docs/api/orders",
        "https://apidocs.chargebee.com/docs/api/payment_reference_numbers",
        "https://apidocs.chargebee.com/docs/api/payment_sources",
        "https://apidocs.chargebee.com/docs/api/payment_vouchers",
        "https://apidocs.chargebee.com/docs/api/promotional_credits",
        "https://apidocs.chargebee.com/docs/api/quoted_charges",
        "https://apidocs.chargebee.com/docs/api/quoted_subscriptions",
        "https://apidocs.chargebee.com/docs/api/quote_line_groups",
        "https://apidocs.chargebee.com/docs/api/quotes",
        "https://apidocs.chargebee.com/docs/api/ramps",
        "https://apidocs.chargebee.com/docs/api/subscription_entitlements",
        "https://apidocs.chargebee.com/docs/api/subscriptions",
        "https://apidocs.chargebee.com/docs/api/transactions",
        "https://apidocs.chargebee.com/docs/api/unbilled_charges",
        "https://apidocs.chargebee.com/docs/api/usages",
        "https://apidocs.chargebee.com/docs/api/virtual_bank_accounts",
    ])
    .map(|url| {
        let client = &client;
        async move {
            println!("{}", url);
            let filename = format!("{}.html", url.split('/').last().unwrap());
            let output_path = Path::new("./.pages/").join(filename);
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
