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
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/account",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/accountlistdetail",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/apagingdetail",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/apagingsummary",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/aragingdetail",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/aragingsummary",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/attachable",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/balancesheet",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/batch",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/bill",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/billpayment",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/budget",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/cashflow",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/changedatacapture",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/class",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/companycurrency",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/companyinfo",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/creditmemo",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/creditcardpayment",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/customer",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/customerbalance",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/customerbalancedetail",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/customerincome",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/fecreport",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/customertype",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/department",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/deposit",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/employee",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/entitlements",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/estimate",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/exchangerate",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/generalledger",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/generalledgerfr",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/inventoryadjustment",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/inventoryvaluationdetail",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/inventoryvaluationsummary",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/invoice",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/item",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/journalcode",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/journalentry",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/journalreport",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/journalreportfr",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/payment",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/paymentmethod",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/preferences",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/profitandloss",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/profitandlossdetail",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/purchase",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/purchaseorder",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/recurringtransaction",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/refundreceipt",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/reimbursecharge",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/salesbyclasssummary",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/salesbycustomer",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/salesbydepartment",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/salesbyproduct",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/salesreceipt",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/taxclassification",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/taxcode",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/taxpayment",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/taxrate",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/taxservice",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/taxsummary",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/taxagency",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/term",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/timeactivity",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/transactionlist",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/transactionlistbyvendor",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/transactionlistbycustomer",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/transactionlistwithsplits",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/transfer",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/trialbalance",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/vendor",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/vendorbalance",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/vendorbalancedetail",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/vendorcredit",
        "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/vendorexpenses",
    ]).map(|url| {
        let client = &client;
        async move {
            println!("{}", url);
            let root = &HttpsClient::scrape_file_path(url, "qbo");
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
