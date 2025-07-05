/*
    Copyright (C) 2017-2025 Yuriy Yarosh.
    All rights reserved.
*/

use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

use bitsnap_accounting_freshbooks_codegen as codegen;

codegen::generate_common_models!();

codegen::generate_model!(
    "BillPayment",
    "https://www.freshbooks.com/api/bill_payments"
);
codegen::generate_model!("BillVendor", "https://www.freshbooks.com/api/vendors");
codegen::generate_model!("Bill", "https://www.freshbooks.com/api/bills");
codegen::generate_model!("Client", "https://www.freshbooks.com/api/clients");
codegen::generate_model!("Estimate", "https://www.freshbooks.com/api/estimates");
codegen::generate_model!("Expense", "https://www.freshbooks.com/api/expenses");
codegen::generate_model!(
    "InvoiceProfile",
    "https://www.freshbooks.com/api/invoice_profiles"
);
codegen::generate_model!("Invoice", "https://www.freshbooks.com/api/invoices");
codegen::generate_model!("Item", "https://www.freshbooks.com/api/items");
codegen::generate_model!("OtherIncome", "https://www.freshbooks.com/api/other_income");
codegen::generate_model!("Payment", "https://www.freshbooks.com/api/payments");
codegen::generate_model!("Project", "https://www.freshbooks.com/api/project");
codegen::generate_model!("Service", "https://www.freshbooks.com/api/services");
codegen::generate_model!("Task", "https://www.freshbooks.com/api/tasks");
codegen::generate_model!("Tax", "https://www.freshbooks.com/api/taxes");
codegen::generate_model!("TeamMember", "https://www.freshbooks.com/api/team-members");
codegen::generate_model!("TimeEntry", "https://www.freshbooks.com/api/time_entries");
