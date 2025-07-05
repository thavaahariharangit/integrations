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
        "https://docs.stripe.com/api/balance",
        "https://docs.stripe.com/api/balance_transactions",
        "https://docs.stripe.com/api/charges",
        "https://docs.stripe.com/api/customers",
        "https://docs.stripe.com/api/customer_sessions",
        "https://docs.stripe.com/api/disputes",
        "https://docs.stripe.com/api/events",
        "https://docs.stripe.com/api/v2/core/events",
        "https://docs.stripe.com/api/v2/core/event_destinations",
        "https://docs.stripe.com/api/files",
        "https://docs.stripe.com/api/file_links",
        "https://docs.stripe.com/api/mandates",
        "https://docs.stripe.com/api/payment_intents",
        "https://docs.stripe.com/api/setup_intents",
        "https://docs.stripe.com/api/setup_attempts",
        "https://docs.stripe.com/api/payouts",
        "https://docs.stripe.com/api/refunds",
        "https://docs.stripe.com/api/confirmation_tokens",
        "https://docs.stripe.com/api/tokens",

        // Payment Methods
        "https://docs.stripe.com/api/payment_methods",
        "https://docs.stripe.com/api/payment_method_configurations",
        "https://docs.stripe.com/api/payment_method_domains",
        "https://docs.stripe.com/api/customer_bank_accounts",
        "https://docs.stripe.com/api/cash_balance",
        "https://docs.stripe.com/api/cash_balance_transactions",
        "https://docs.stripe.com/api/cards",
        "https://docs.stripe.com/api/sources", // Deprecated

        // Products
        "https://docs.stripe.com/api/products",
        "https://docs.stripe.com/api/prices",
        "https://docs.stripe.com/api/coupons",
        "https://docs.stripe.com/api/promotion_codes",
        "https://docs.stripe.com/api/discounts",
        "https://docs.stripe.com/api/tax_codes",
        "https://docs.stripe.com/api/tax_rates",
        "https://docs.stripe.com/api/shipping_rates",

        // Checkout Sessions
        "https://docs.stripe.com/api/checkout/sessions",


        // Payment Link
        "https://docs.stripe.com/api/payment-link",

        // Billing
        "https://docs.stripe.com/api/credit_notes",
        "https://docs.stripe.com/api/customer_balance_transactions",
        "https://docs.stripe.com/api/customer_portal/sessions",
        "https://docs.stripe.com/api/customer_portal/configurations",
        "https://docs.stripe.com/api/invoices",
        "https://docs.stripe.com/api/invoiceitems",
        "https://docs.stripe.com/api/invoice-line-item",
        "https://docs.stripe.com/api/invoice-rendering-template",
        "https://docs.stripe.com/api/billing/alert",
        "https://docs.stripe.com/api/billing/meter",
        "https://docs.stripe.com/api/billing/meter-event",
        "https://docs.stripe.com/api/billing/meter-event-adjustment",
        "https://docs.stripe.com/api/billing/meter-event-summary",
        "https://docs.stripe.com/api/v2/billing-meter",
        "https://docs.stripe.com/api/v2/billing-meter-adjustment",
        "https://docs.stripe.com/api/v2/billing-meter-stream",
        "https://docs.stripe.com/api/billing/credit-grant",
        "https://docs.stripe.com/api/billing/credit-balance-summary",
        "https://docs.stripe.com/api/billing/credit-balance-transaction",
        "https://docs.stripe.com/api/plans",
        "https://docs.stripe.com/api/quotes",
        "https://docs.stripe.com/api/subscriptions",
        "https://docs.stripe.com/api/subscription_items",
        "https://docs.stripe.com/api/subscription_schedules",
        "https://docs.stripe.com/api/tax_ids",
        "https://docs.stripe.com/api/test_clocks", // TEST HELPER
        "https://docs.stripe.com/api/usage_records",
        "https://docs.stripe.com/api/usage-record-summary",

        // Capital
        "https://docs.stripe.com/api/capital/financing_offers",
        "https://docs.stripe.com/api/capital/financing_summary",

        // Connect
        "https://docs.stripe.com/api/accounts",
        "https://docs.stripe.com/api/accounts/login_link",
        "https://docs.stripe.com/api/account_links",
        "https://docs.stripe.com/api/account_sessions",
        "https://docs.stripe.com/api/application_fees",
        "https://docs.stripe.com/api/fee_refunds",
        "https://docs.stripe.com/api/capabilities",
        "https://docs.stripe.com/api/country_specs",
        "https://docs.stripe.com/api/external_accounts",
        "https://docs.stripe.com/api/external_account_cards",
        "https://docs.stripe.com/api/persons",
        "https://docs.stripe.com/api/topups",
        "https://docs.stripe.com/api/transfers",
        "https://docs.stripe.com/api/transfer_reversals",
        "https://docs.stripe.com/api/secret_management",

        // Fraud
        "https://docs.stripe.com/api/radar/early_fraud_warnings",
        "https://docs.stripe.com/api/radar/reviews",
        "https://docs.stripe.com/api/radar/value_lists",
        "https://docs.stripe.com/api/radar/value_list_items",

        // Issuing
        "https://docs.stripe.com/api/issuing/authorizations",
        "https://docs.stripe.com/api/issuing/cardholders",
        "https://docs.stripe.com/api/issuing/cards",
        "https://docs.stripe.com/api/issuing/disputes",
        "https://docs.stripe.com/api/issuing/funding_instructions",
        "https://docs.stripe.com/api/issuing/personalization_designs",
        "https://docs.stripe.com/api/issuing/physical_bundles",
        "https://docs.stripe.com/api/issuing/tokens",
        "https://docs.stripe.com/api/issuing/transactions",

        // Terminal
        "https://docs.stripe.com/api/terminal/connection_tokens",
        "https://docs.stripe.com/api/terminal/locations",
        "https://docs.stripe.com/api/terminal/readers",
        "https://docs.stripe.com/api/terminal/hardware_orders",
        "https://docs.stripe.com/api/terminal/hardware_products",
        "https://docs.stripe.com/api/terminal/hardware_skus",
        "https://docs.stripe.com/api/terminal/hardware_shipping_methods",
        "https://docs.stripe.com/api/terminal/configuration",

        // Treasury
        "https://docs.stripe.com/api/treasury/financial_accounts",
        "https://docs.stripe.com/api/treasury/financial_account_features",
        "https://docs.stripe.com/api/treasury/transactions",
        "https://docs.stripe.com/api/treasury/transaction_entries",
        "https://docs.stripe.com/api/treasury/outbound_transfers",
        "https://docs.stripe.com/api/treasury/outbound_payments",
        "https://docs.stripe.com/api/treasury/inbound_transfers",
        "https://docs.stripe.com/api/treasury/received_credits",
        "https://docs.stripe.com/api/treasury/received_debits",
        "https://docs.stripe.com/api/treasury/credit_reversals",
        "https://docs.stripe.com/api/treasury/debit_reversals",

        // entitlements
        "https://docs.stripe.com/api/entitlements/feature",
        "https://docs.stripe.com/api/product-feature",
        "https://docs.stripe.com/api/entitlements/active-entitlement",

        // Sigma
        "https://docs.stripe.com/api/sigma/scheduled_queries",

        // Reporting
        "https://docs.stripe.com/api/reporting/report_run",
        "https://docs.stripe.com/api/reporting/report_type",

        // Financial connections
        "https://docs.stripe.com/api/financial_connections/accounts",
        "https://docs.stripe.com/api/financial_connections/ownership",
        "https://docs.stripe.com/api/financial_connections/sessions",
        "https://docs.stripe.com/api/financial_connections/transactions",

        // Tax
        "https://docs.stripe.com/api/tax/calculations",
        "https://docs.stripe.com/api/tax/registrations",
        "https://docs.stripe.com/api/tax/transactions",
        "https://docs.stripe.com/api/tax/settings",

        // Identity
        "https://docs.stripe.com/api/identity/verification_sessions",
        "https://docs.stripe.com/api/identity/verification_reports",

        // Crypto
        "https://docs.stripe.com/api/crypto/onramp_sessions",
        "https://docs.stripe.com/api/crypto/onramp_quotes",

        // Climate
        "https://docs.stripe.com/api/climate/order",
        "https://docs.stripe.com/api/climate/product",
        "https://docs.stripe.com/api/climate/supplier",

        // Forwarding
        "https://docs.stripe.com/api/forwarding/request",

        // Webhook endpoints
        "https://docs.stripe.com/api/webhook_endpoints",
    ]).map(|url| {
        let client = &client;
        async move {
            println!("{}", url);
            let root = &HttpsClient::scrape_file_path(url, "stripe");
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
