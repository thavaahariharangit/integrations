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

        "https://developer.xero.com/documentation/api/accounting/accounts",
        "https://developer.xero.com/documentation/api/accounting/attachments",
        "https://developer.xero.com/documentation/api/accounting/bankstatements",
        "https://developer.xero.com/documentation/api/accounting/banktransactions",
        "https://developer.xero.com/documentation/api/accounting/banktransfers",
        "https://developer.xero.com/documentation/api/accounting/batchpayments",
        "https://developer.xero.com/documentation/api/accounting/brandingthemes",
        "https://developer.xero.com/documentation/api/accounting/budgets",
        "https://developer.xero.com/documentation/api/accounting/contactgroups",
        "https://developer.xero.com/documentation/api/accounting/contacts",
        "https://developer.xero.com/documentation/api/accounting/creditnotes",
        "https://developer.xero.com/documentation/api/accounting/currencies",
        "https://developer.xero.com/documentation/api/accounting/employees",
        "https://developer.xero.com/documentation/api/accounting/expenseclaims", // Deprecated
        "https://developer.xero.com/documentation/api/accounting/requests-and-responses", // Deprecated
        "https://developer.xero.com/documentation/api/accounting/historyandnotes",
        "https://developer.xero.com/documentation/api/accounting/invoicereminders",
        "https://developer.xero.com/documentation/api/accounting/invoices",
        "https://developer.xero.com/documentation/api/accounting/items",
        "https://developer.xero.com/documentation/api/accounting/journals",
        "https://developer.xero.com/documentation/api/accounting/linkedtransactions",
        "https://developer.xero.com/documentation/api/accounting/manualjournals",
        "https://developer.xero.com/documentation/api/accounting/organisation",
        "https://developer.xero.com/documentation/api/accounting/overpayments",
        "https://developer.xero.com/documentation/api/accounting/paymentservices",
        "https://developer.xero.com/documentation/api/accounting/payments",
        "https://developer.xero.com/documentation/api/accounting/prepayments",
        "https://developer.xero.com/documentation/api/accounting/purchaseorders",
        "https://developer.xero.com/documentation/api/accounting/quotes",
        "https://developer.xero.com/documentation/api/accounting/receipts", // Deprecated
        "https://developer.xero.com/documentation/api/accounting/releasenotes",
        "https://developer.xero.com/documentation/api/accounting/repeatinginvoices",
        "https://developer.xero.com/documentation/api/accounting/reports",
        "https://developer.xero.com/documentation/api/accounting/responsecodes",
        "https://developer.xero.com/documentation/api/accounting/taxrates",
        "https://developer.xero.com/documentation/api/accounting/trackingcategories",
        "https://developer.xero.com/documentation/api/accounting/users",

        // Accounting enums
        "https://developer.xero.com/documentation/api/accounting/types",

        // Assets API
        "https://developer.xero.com/documentation/api/assets/assettypes",
        "https://developer.xero.com/documentation/api/assets/assets",
        "https://developer.xero.com/documentation/api/assets/settings",

        // Assets enums
        "https://developer.xero.com/documentation/api/assets/types",

        // Files API
        "https://developer.xero.com/documentation/api/files/associations",
        "https://developer.xero.com/documentation/api/files/files",
        "https://developer.xero.com/documentation/api/files/folders",

        // Files enums
        "https://developer.xero.com/documentation/api/files/types",

        // Finance API
        "https://developer.xero.com/documentation/api/finance/accountingactivities",
        "https://developer.xero.com/documentation/api/finance/bankstatementsplus",
        "https://developer.xero.com/documentation/api/finance/cashvalidation",
        "https://developer.xero.com/documentation/api/finance/financialstatements",

        // Payroll AU
        "https://developer.xero.com/documentation/api/payrollau/employees",
        "https://developer.xero.com/documentation/api/payrollau/leaveapplications",
        "https://developer.xero.com/documentation/api/payrollau/leavebalances",
        "https://developer.xero.com/documentation/api/payrollau/payitems",
        "https://developer.xero.com/documentation/api/payrollau/payruns",
        "https://developer.xero.com/documentation/api/payrollau/payrollcalendars",
        "https://developer.xero.com/documentation/api/payrollau/payslip",
        "https://developer.xero.com/documentation/api/payrollau/stp-changes",
        "https://developer.xero.com/documentation/api/payrollau/settings",
        "https://developer.xero.com/documentation/api/payrollau/superfundproducts",
        "https://developer.xero.com/documentation/api/payrollau/superfunds",
        "https://developer.xero.com/documentation/api/payrollau/timesheets",

        // ENUMS
        "https://developer.xero.com/documentation/api/payrollau/types-and-codes",

        // Payroll NZ

        "https://developer.xero.com/documentation/api/payrollnz/deductions",
        "https://developer.xero.com/documentation/api/payrollnz/earningsrates",
        "https://developer.xero.com/documentation/api/payrollnz/employeeworkingpatterns",
        "https://developer.xero.com/documentation/api/payrollnz/employees",
        "https://developer.xero.com/documentation/api/payrollnz/employeeleave",
        "https://developer.xero.com/documentation/api/payrollnz/employeeleaveperiods",
        "https://developer.xero.com/documentation/api/payrollnz/employeeleavesetup",
        "https://developer.xero.com/documentation/api/payrollnz/employeeleavetypes",
        "https://developer.xero.com/documentation/api/payrollnz/employeeopeningbalances",
        "https://developer.xero.com/documentation/api/payrollnz/employeepaytemplates",
        "https://developer.xero.com/documentation/api/payrollnz/employeetax",

        "https://developer.xero.com/documentation/api/payrollnz/employment",
        "https://developer.xero.com/documentation/api/payrollnz/leavebalances",

        "https://developer.xero.com/documentation/api/payrollnz/employeeleavetypes",
        "https://developer.xero.com/documentation/api/payrollnz/payruncalendars",
        "https://developer.xero.com/documentation/api/payrollnz/paymentmethods",
        "https://developer.xero.com/documentation/api/payrollnz/payruns",
        "https://developer.xero.com/documentation/api/payrollnz/payslips",
        "https://developer.xero.com/documentation/api/payrollnz/reimbursements",
        "https://developer.xero.com/documentation/api/payrollnz/salaryandwages",
        "https://developer.xero.com/documentation/api/payrollnz/settings",
        "https://developer.xero.com/documentation/api/payrollnz/statutorydeductions",
        "https://developer.xero.com/documentation/api/payrollnz/superannuation",
        "https://developer.xero.com/documentation/api/payrollnz/timesheets",
        "https://developer.xero.com/documentation/api/payrollnz/trackingcategories",

        "https://developer.xero.com/documentation/api/payrollnz/nztaxcodes",
        "https://developer.xero.com/documentation/api/payrollnz/typesandcodes",

        // Payroll UK
        "https://developer.xero.com/documentation/api/payrolluk/deductions",
        "https://developer.xero.com/documentation/api/payrolluk/earningrates",
        "https://developer.xero.com/documentation/api/payrolluk/earningsorders",
        "https://developer.xero.com/documentation/api/payrolluk/employeeopeningbalances",
        "https://developer.xero.com/documentation/api/payrolluk/employeeworkingpatterns",
        "https://developer.xero.com/documentation/api/payrolluk/employees",
        "https://developer.xero.com/documentation/api/payrolluk/employeeleave",
        "https://developer.xero.com/documentation/api/payrolluk/employeeleavebalances",
        "https://developer.xero.com/documentation/api/payrolluk/employeeleaveperiods",
        "https://developer.xero.com/documentation/api/payrolluk/employeeleavetypes",
        "https://developer.xero.com/documentation/api/payrolluk/employeepaytemplates",
        "https://developer.xero.com/documentation/api/payrolluk/employeestatutoryleavebalance",
        "https://developer.xero.com/documentation/api/payrolluk/employeestatutoryleavessummary",
        "https://developer.xero.com/documentation/api/payrolluk/employeestatutorysickleave",
        "https://developer.xero.com/documentation/api/payrolluk/employeetax",
        "https://developer.xero.com/documentation/api/payrolluk/employerpensions",
        "https://developer.xero.com/documentation/api/payrolluk/employment",
        "https://developer.xero.com/documentation/api/payrolluk/leavetypes",
        "https://developer.xero.com/documentation/api/payrolluk/payruncalendars",
        "https://developer.xero.com/documentation/api/payrolluk/payruns",
        "https://developer.xero.com/documentation/api/payrolluk/paymentmethods",
        "https://developer.xero.com/documentation/api/payrolluk/payslips",
        "https://developer.xero.com/documentation/api/payrolluk/reimbursements",
        "https://developer.xero.com/documentation/api/payrolluk/salaryandwages",
        "https://developer.xero.com/documentation/api/payrolluk/settings",
        "https://developer.xero.com/documentation/api/payrolluk/timesheets",
        "https://developer.xero.com/documentation/api/payrolluk/trackingcategories",
        "https://developer.xero.com/documentation/api/payrolluk/typesandcodes",
        // Projects
        "https://developer.xero.com/documentation/api/projects/projects",
        "https://developer.xero.com/documentation/api/projects/tasks",
        "https://developer.xero.com/documentation/api/projects/time",
        "https://developer.xero.com/documentation/api/projects/users",

    ]).map(|url| {
        let client = &client;
        async move {
            println!("{}", url);
            let root = &HttpsClient::scrape_file_path(url, "xero");
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
