/*
    Copyright (C) 2017-2025 Yuriy Yarosh.
    All rights reserved.
*/

use bitsnap_accounting_qbo_codegen as codegen;

codegen::generate_model!("Account", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/account");
codegen::generate_model!("AccountListDetail", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/accountlistdetail");

codegen::generate_model!("ApAgingDetail", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/apagingdetail");
codegen::generate_model!("ApAgingSummary", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/apagingsummary");

codegen::generate_model!("ArAgingDetail", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/aragingdetail");
codegen::generate_model!("ArAgingSummary", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/aragingsummary");

codegen::generate_model!("Attachable", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/attachable");
codegen::generate_model!("BalanceSheet", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/balancesheet");
codegen::generate_model!("Batch", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/batch");

codegen::generate_model!("Bill", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/bill");
codegen::generate_model!("BillPayment", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/billpayment");

codegen::generate_model!("Budget", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/budget");
codegen::generate_model!("CashFlow", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/cashflow");
codegen::generate_model!("ChangeDataCapture", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/changedatacapture");

codegen::generate_model!("Class", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/class");

codegen::generate_model!("CompanyCurrency", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/companycurrency");
codegen::generate_model!("CompanyInfo", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/companyinfo");
codegen::generate_model!("CreditMemo", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/creditmemo");

codegen::generate_model!("CreditCardPayment", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/creditcardpayment");
codegen::generate_model!("Customer", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/customer");
codegen::generate_model!("CustomerBalance", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/customerbalance");

codegen::generate_model!("CustomerBalanceDetail", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/customerbalancedetail");
codegen::generate_model!("CustomerIncome", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/customerincome");
codegen::generate_model!("FECReport", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/fecreport");
codegen::generate_model!("CustomerType", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/customertype");
codegen::generate_model!("Department", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/department");
codegen::generate_model!("Deposit", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/deposit");
codegen::generate_model!("Employee", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/employee");
codegen::generate_model!("Entitlements", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/entitlements");
codegen::generate_model!("Estimate", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/estimate");
codegen::generate_model!("ExchangeRate", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/exchangerate");
codegen::generate_model!("GeneralLedger", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/generalledger");
codegen::generate_model!("GeneralLedgerFR", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/generalledgerfr");

codegen::generate_model!("InventoryAdjustment", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/inventoryadjustment");
codegen::generate_model!("InventoryValuationDetail", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/inventoryvaluationdetail");
codegen::generate_model!("InventoryValuationSummary", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/inventoryvaluationsummary");

codegen::generate_model!("Invoice", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/invoice");
codegen::generate_model!("Item", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/item");

codegen::generate_model!("JournalCode", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/journalcode");
codegen::generate_model!("JournalEntry", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/journalentry");

codegen::generate_model!("JournalReport", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/journalreport");
codegen::generate_model!("JournalReportFR", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/journalreportfr");

codegen::generate_model!("Payment", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/payment");
codegen::generate_model!("PaymentMethod", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/paymentmethod");

codegen::generate_model!("Preferences", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/preferences");
codegen::generate_model!("ProfitAndLoss", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/profitandloss");
codegen::generate_model!("ProfitAndLossDetail", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/profitandlossdetail");
codegen::generate_model!("Purchase", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/purchase");
codegen::generate_model!("PurchaseOrder", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/purchaseorder");

codegen::generate_model!("RecurringTransaction", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/recurringtransaction");
codegen::generate_model!("RefundReceipt", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/refundreceipt");
codegen::generate_model!("ReimburseCharge", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/reimbursecharge");

codegen::generate_model!("SalesByClassSummary", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/salesbyclasssummary");
codegen::generate_model!("SalesByCustomer", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/salesbycustomer");
codegen::generate_model!("SalesByDepartment", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/salesbydepartment");
codegen::generate_model!("SalesByProduct", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/salesbyproduct");
codegen::generate_model!("SalesReceipt", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/salesreceipt");

codegen::generate_model!("TaxClassification", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/taxclassification");
codegen::generate_model!("TaxCode", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/taxcode");
codegen::generate_model!("TaxPayment", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/taxpayment");
codegen::generate_model!("TaxRate", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/taxrate");
codegen::generate_model!("TaxService", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/taxservice");
codegen::generate_model!("TaxSummary", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/taxsummary");
codegen::generate_model!("TaxAgency", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/taxagency");

codegen::generate_model!("Term", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/term");
codegen::generate_model!("TimeActivity", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/timeactivity");

codegen::generate_model!("TransactionList", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/transactionlist");
codegen::generate_model!("TransactionListByVendor", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/transactionlistbyvendor");
codegen::generate_model!("TransactionListByCustomer", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/transactionlistbycustomer");
codegen::generate_model!("TransactionListWithSplits", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/transactionlistwithsplits");


codegen::generate_model!("Transfer", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/transfer");
codegen::generate_model!("TrialBalance", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/trialbalance");

codegen::generate_model!("Vendor", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/vendor");
codegen::generate_model!("VendorBalance", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/vendorbalance");
codegen::generate_model!("VendorBalanceDetail", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/vendorbalancedetail");
codegen::generate_model!("VendorCredit", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/vendorcredit");
codegen::generate_model!("VendorExpenses", "https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/vendorexpenses");
