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
        "https://developer.myob.com/api/myob-business-api/v2/company-files",
        "https://developer.myob.com/api/myob-business-api/v2/info",
        "https://developer.myob.com/api/myob-business-api/v2/current-user",
        // Contact
        "https://developer.myob.com/api/myob-business-api/v2/contact",
        "https://developer.myob.com/api/myob-business-api/v2/contact/customer",
        "https://developer.myob.com/api/myob-business-api/v2/contact/supplier",
        "https://developer.myob.com/api/myob-business-api/v2/contact/employee",
        "https://developer.myob.com/api/myob-business-api/v2/contact/employee-payroll-details",
        "https://developer.myob.com/api/myob-business-api/v2/contact/employee-payment-details",
        "https://developer.myob.com/api/myob-business-api/v2/contact/employee-standard-pay",
        "https://developer.myob.com/api/myob-business-api/v2/contact/personal",
        // GeneralLedger
        "https://developer.myob.com/api/myob-business-api/v2/generalledger/account",
        "https://developer.myob.com/api/myob-business-api/v2/generalledger/accountbudget",
        "https://developer.myob.com/api/myob-business-api/v2/generalledger/accountregister",
        "https://developer.myob.com/api/myob-business-api/v2/generalledger/accountingproperties",

        "https://developer.myob.com/api/myob-business-api/v2/generalledger/category",
        "https://developer.myob.com/api/myob-business-api/v2/generalledger/categoryregister",
        "https://developer.myob.com/api/myob-business-api/v2/generalledger/currency",
        "https://developer.myob.com/api/myob-business-api/v2/generalledger/generaljournal",

        "https://developer.myob.com/api/myob-business-api/v2/generalledger/job",
        "https://developer.myob.com/api/myob-business-api/v2/generalledger/jobbudget",
        "https://developer.myob.com/api/myob-business-api/v2/generalledger/jobregister",

        "https://developer.myob.com/api/myob-business-api/v2/generalledger/journaltransaction",
        "https://developer.myob.com/api/myob-business-api/v2/generalledger/linkedaccount",
        "https://developer.myob.com/api/myob-business-api/v2/generalledger/profitloss-distribution",
        "https://developer.myob.com/api/myob-business-api/v2/generalledger/taxcode",

        // Inventory
        "https://developer.myob.com/api/myob-business-api/v2/inventory/item",
        "https://developer.myob.com/api/myob-business-api/v2/inventory/item/bill-materials",
        "https://developer.myob.com/api/myob-business-api/v2/inventory/item_price_matrix",
        "https://developer.myob.com/api/myob-business-api/v2/inventory/item_price_matrix/price_level_detail",
        "https://developer.myob.com/api/myob-business-api/v2/inventory/adjustment",
        "https://developer.myob.com/api/myob-business-api/v2/inventory/location",
        "https://developer.myob.com/api/myob-business-api/v2/inventory/build",


        // Sale
        "https://developer.myob.com/api/myob-business-api/v2/sale/customerpayment",
        "https://developer.myob.com/api/myob-business-api/v2/sale/customerpayment/calculate_discounts_fees",
        "https://developer.myob.com/api/myob-business-api/v2/sale/customerpayment/record_with_discounts_fees",
        "https://developer.myob.com/api/myob-business-api/v2/sale/credit_settlement",
        "https://developer.myob.com/api/myob-business-api/v2/sale/credit_refund",

        "https://developer.myob.com/api/myob-business-api/v2/sale/invoice",
        "https://developer.myob.com/api/myob-business-api/v2/sale/invoice/invoice_item",
        "https://developer.myob.com/api/myob-business-api/v2/sale/invoice/invoice_item/item-invoice-email",
        "https://developer.myob.com/api/myob-business-api/v2/sale/invoice/invoice_service",
        "https://developer.myob.com/api/myob-business-api/v2/sale/invoice/invoice_service/service-invoice-email",
        "https://developer.myob.com/api/myob-business-api/v2/sale/invoice/invoice_professional",
        "https://developer.myob.com/api/myob-business-api/v2/sale/invoice/invoice_professional/professional-invoice-email",
        "https://developer.myob.com/api/myob-business-api/v2/sale/invoice/invoice_timebilling",
        "https://developer.myob.com/api/myob-business-api/v2/sale/invoice/invoice_timebilling/timebilling-invoice-email",
        "https://developer.myob.com/api/myob-business-api/v2/sale/invoice/invoice_miscellaneous",
        "https://developer.myob.com/api/myob-business-api/v2/sale/invoice/invoice_miscellaneous/miscellaneous-invoice-email",

        "https://developer.myob.com/api/myob-business-api/v2/sale/order",
        "https://developer.myob.com/api/myob-business-api/v2/sale/order/order_item",
        "https://developer.myob.com/api/myob-business-api/v2/sale/order/order_miscellaneous",
        "https://developer.myob.com/api/myob-business-api/v2/sale/order/order_professional",
        "https://developer.myob.com/api/myob-business-api/v2/sale/order/order_service",
        "https://developer.myob.com/api/myob-business-api/v2/sale/order/order_timebilling",

        "https://developer.myob.com/api/myob-business-api/v2/sale/quote",
        "https://developer.myob.com/api/myob-business-api/v2/sale/quote/quote_item",
        "https://developer.myob.com/api/myob-business-api/v2/sale/quote/quote_miscellaneous",
        "https://developer.myob.com/api/myob-business-api/v2/sale/quote/quote_professional",
        "https://developer.myob.com/api/myob-business-api/v2/sale/quote/quotes_service",
        "https://developer.myob.com/api/myob-business-api/v2/sale/quote/quote_timebilling",

        // Purchase
        "https://developer.myob.com/api/myob-business-api/v2/purchase/supplierpayment",

        "https://developer.myob.com/api/myob-business-api/v2/purchase/supplierpayment/calculate_discounts",
        "https://developer.myob.com/api/myob-business-api/v2/purchase/supplierpayment/record_with_discounts_fees",

        "https://developer.myob.com/api/myob-business-api/v2/purchase/debit_refund",
        "https://developer.myob.com/api/myob-business-api/v2/purchase/debit_settlement",

        "https://developer.myob.com/api/myob-business-api/v2/purchase/bill",
        "https://developer.myob.com/api/myob-business-api/v2/purchase/bill/bill_item",
        "https://developer.myob.com/api/myob-business-api/v2/purchase/bill/bill_item/item-bill-attachment",
        "https://developer.myob.com/api/myob-business-api/v2/purchase/bill/bill_service",
        "https://developer.myob.com/api/myob-business-api/v2/purchase/bill/bill_service/service-bill-attachment",
        "https://developer.myob.com/api/myob-business-api/v2/purchase/bill/bill_miscellaneous",
        "https://developer.myob.com/api/myob-business-api/v2/purchase/bill/bill_miscellaneous/miscellaneous-bill-attachment",
        "https://developer.myob.com/api/myob-business-api/v2/purchase/bill/bill_professional",
        "https://developer.myob.com/api/myob-business-api/v2/purchase/bill/bill_professional/professional-bill-attachment",

        "https://developer.myob.com/api/myob-business-api/v2/purchase/order",
        "https://developer.myob.com/api/myob-business-api/v2/purchase/order/order_item",
        "https://developer.myob.com/api/myob-business-api/v2/purchase/order/order_miscellaneous",
        "https://developer.myob.com/api/myob-business-api/v2/purchase/order/order_professional",
        "https://developer.myob.com/api/myob-business-api/v2/purchase/order/order_service",

        // Banking
        "https://developer.myob.com/api/myob-business-api/v2/banking/spend_money",
        "https://developer.myob.com/api/myob-business-api/v2/banking/spend_money/spend-money-attachments",
        "https://developer.myob.com/api/myob-business-api/v2/banking/receive_money",
        "https://developer.myob.com/api/myob-business-api/v2/banking/transfer_money",
        "https://developer.myob.com/api/myob-business-api/v2/banking/statement",
        "https://developer.myob.com/api/myob-business-api/v2/banking/bank-account",

        // Payroll
        "https://developer.myob.com/api/myob-business-api/v2/payroll/payroll-category",
        "https://developer.myob.com/api/myob-business-api/v2/payroll/payroll-category/wage",
        "https://developer.myob.com/api/myob-business-api/v2/payroll/payroll-category/entitlement",
        "https://developer.myob.com/api/myob-business-api/v2/payroll/payroll-category/deduction",
        "https://developer.myob.com/api/myob-business-api/v2/payroll/payroll-category/expense",
        "https://developer.myob.com/api/myob-business-api/v2/payroll/payroll-category/superannuation",
        "https://developer.myob.com/api/myob-business-api/v2/payroll/payroll-category/tax",
        "https://developer.myob.com/api/myob-business-api/v2/payroll/payroll-category/tax-table",

        "https://developer.myob.com/api/myob-business-api/v2/payroll/superannuationfund",
        "https://developer.myob.com/api/myob-business-api/v2/payroll/employment-classification",
        "https://developer.myob.com/api/myob-business-api/v2/payroll/timesheet",

        // Timebilling
        "https://developer.myob.com/api/myob-business-api/v2/timebilling/activity",
        "https://developer.myob.com/api/myob-business-api/v2/timebilling/activity-slip",

        // Company
        "https://developer.myob.com/api/myob-business-api/v2/company/custom-list",
        "https://developer.myob.com/api/myob-business-api/v2/company/form_template",
        "https://developer.myob.com/api/myob-business-api/v2/company/preferences",
        "https://developer.myob.com/api/myob-business-api/v2/company",
    ]).map(|url| {
        let client = &client;
        async move {
            println!("{}", url);
            let root = &HttpsClient::scrape_file_path(url, "myob");
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
