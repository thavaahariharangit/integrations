/*
    Copyright (C) 2017-2025 Yuriy Yarosh.
    All rights reserved.
*/

use bitsnap_accounting_myob_codegen as codegen;

codegen::generate_model!(
    "CompanyFiles",
    "https://developer.myob.com/api/myob-business-api/v2/company-files"
); // {domain}/accountright/
codegen::generate_model!(
    "Info",
    "https://developer.myob.com/api/myob-business-api/v2/info"
); // {domain}/accountright/Info
codegen::generate_model!(
    "User",
    "https://developer.myob.com/api/myob-business-api/v2/current-user"
); // /{cf_uri}/CurrentUser

// Contact
codegen::generate_model!(
    "Contact",
    "https://developer.myob.com/api/myob-business-api/v2/contact"
); // /{cf_uri}/Contact /{cf_uri}/Contact/{contact_uid}/Photo
codegen::generate_model!(
    "CustomerContact",
    "https://developer.myob.com/api/myob-business-api/v2/contact/customer"
); // /{cf_uri}/Contact/Customer /{cf_uri}/Contact/Customer/{contact_uid}/Photo
codegen::generate_model!(
    "SupplierContact",
    "https://developer.myob.com/api/myob-business-api/v2/contact/supplier"
); // /{cf_uri}/Contact/Supplier /{cf_uri}/Contact/Supplier/{contact_uid}/Photo
codegen::generate_model!(
    "EmployeeContact",
    "https://developer.myob.com/api/myob-business-api/v2/contact/employee"
); // /{cf_uri}/Contact/Employee /{cf_uri}/Contact/Employee/{contact_uid}/Photo
codegen::generate_model!(
    "EmployeePayrollDetails",
    "https://developer.myob.com/api/myob-business-api/v2/contact/employee-payroll-details"
); // /{cf_uri}/Contact/EmployeePayrollDetails
codegen::generate_model!(
    "EmployeePaymentDetails",
    "https://developer.myob.com/api/myob-business-api/v2/contact/employee-payment-details"
); // /{cf_uri}/Contact/EmployeePaymentDetails
codegen::generate_model!(
    "EmployeeStandardPay",
    "https://developer.myob.com/api/myob-business-api/v2/contact/employee-standard-pay"
); // /{cf_uri}/Contact/EmployeeStandardPay
codegen::generate_model!(
    "PersonalContact",
    "https://developer.myob.com/api/myob-business-api/v2/contact/personal"
); // /{cf_uri}/Contact/Personal /{cf_uri}/Contact/Personal{contact_uid}/Photo

// GeneralLedger
codegen::generate_model!(
    "LedgerAccount",
    "https://developer.myob.com/api/myob-business-api/v2/generalledger/account"
); // /{cf_uri}/GeneralLedger/Account
codegen::generate_model!(
    "LedgerAccountBudget",
    "https://developer.myob.com/api/myob-business-api/v2/generalledger/accountbudget"
); // /{cf_uri}/GeneralLedger/AccountBudget
codegen::generate_model!(
    "LedgerAccountRegister",
    "https://developer.myob.com/api/myob-business-api/v2/generalledger/accountregister"
); // /{cf_uri}/GeneralLedger/AccountRegister
codegen::generate_model!(
    "LedgerAccountingProperties",
    "https://developer.myob.com/api/myob-business-api/v2/generalledger/accountingproperties"
); // /{cf_uri}/GeneralLedger/AccountingProperties

codegen::generate_model!(
    "LedgerCategory",
    "https://developer.myob.com/api/myob-business-api/v2/generalledger/category"
); // /{cf_uri}/GeneralLedger/Category
codegen::generate_model!(
    "LedgerCategoryRegister",
    "https://developer.myob.com/api/myob-business-api/v2/generalledger/categoryregister"
); // /{cf_uri}/GeneralLedger/CategoryRegister
codegen::generate_model!(
    "LedgerCurrency",
    "https://developer.myob.com/api/myob-business-api/v2/generalledger/currency"
); // /{cf_uri}/GeneralLedger/Currency
codegen::generate_model!(
    "LedgerGeneralJournal",
    "https://developer.myob.com/api/myob-business-api/v2/generalledger/generaljournal"
); // /{cf_uri}/GeneralLedger/GeneralJournal

codegen::generate_model!(
    "LedgerJob",
    "https://developer.myob.com/api/myob-business-api/v2/generalledger/job"
); // /{cf_uri}/GeneralLedger/Job
codegen::generate_model!(
    "LedgerJobBudget",
    "https://developer.myob.com/api/myob-business-api/v2/generalledger/jobbudget"
); // /{cf_uri}/GeneralLedger/JobBudget
codegen::generate_model!(
    "LedgerJobRegister",
    "https://developer.myob.com/api/myob-business-api/v2/generalledger/jobregister"
); // /{cf_uri}/GeneralLedger/JobRegister

codegen::generate_model!(
    "LedgerJournalTransaction",
    "https://developer.myob.com/api/myob-business-api/v2/generalledger/journaltransaction"
); // /{cf_uri}/GeneralLedger/JournalTransaction
codegen::generate_model!(
    "LedgerLinkedAccount",
    "https://developer.myob.com/api/myob-business-api/v2/generalledger/linkedaccount"
); // /{cf_uri}/GeneralLedger/LinkedAccount
codegen::generate_model!(
    "LedgerProfitLossDistribution",
    "https://developer.myob.com/api/myob-business-api/v2/generalledger/profitloss-distribution"
); // /{cf_uri}/GeneralLedger/ProfitLossDistribution
codegen::generate_model!(
    "LedgerTaxCode",
    "https://developer.myob.com/api/myob-business-api/v2/generalledger/taxcode"
); // /{cf_uri}/GeneralLedger/TaxCode

// Inventory
codegen::generate_model!(
    "InventoryItem",
    "https://developer.myob.com/api/myob-business-api/v2/inventory/item"
); // /{cf_uri}/Inventory/Item
codegen::generate_model!(
    "InventoryItemWithBillOfMaterials",
    "https://developer.myob.com/api/myob-business-api/v2/inventory/item/bill-materials"
); // /{cf_uri}/Inventory/Item/?$expand=BillOfMaterials
codegen::generate_model!(
    "InventoryItemPriceMatrix",
    "https://developer.myob.com/api/myob-business-api/v2/inventory/item_price_matrix"
); // /{cf_uri}/Inventory/ItemPriceMatrix
codegen::generate_model!(
    "InventoryItemPriceLevelDetail",
    "https://developer.myob.com/api/myob-business-api/v2/inventory/item_price_matrix/price_level_detail"
); // /{cf_uri}/Inventory/PriceLevelDetail
codegen::generate_model!(
    "InventoryAdjustment",
    "https://developer.myob.com/api/myob-business-api/v2/inventory/adjustment"
); // /{cf_uri}/Inventory/Adjustment
codegen::generate_model!(
    "InventoryLocation",
    "https://developer.myob.com/api/myob-business-api/v2/inventory/location"
); // /{cf_uri}/Inventory/Location
codegen::generate_model!(
    "InventoryBuild",
    "https://developer.myob.com/api/myob-business-api/v2/inventory/build"
); // /{cf_uri}/Inventory/Build

// Sale
codegen::generate_model!(
    "CustomerPayment",
    "https://developer.myob.com/api/myob-business-api/v2/sale/customerpayment"
); // /{cf_uri}/Sale/CustomerPayment
codegen::generate_model!(
    "CustomerPaymentCalculateDiscountsFees",
    "https://developer.myob.com/api/myob-business-api/v2/sale/customerpayment/calculate_discounts_fees"
); // /{cf_uri}/Sale/CustomerPayment/CalculateDiscountsAndFees
codegen::generate_model!(
    "CustomerPaymentRecordWithDiscountsAndFees",
    "https://developer.myob.com/api/myob-business-api/v2/sale/customerpayment/record_with_discounts_fees"
); // /{cf_uri}/Sale/CustomerPayment/RecordWithDiscountsAndFees
codegen::generate_model!(
    "CreditSettlement",
    "https://developer.myob.com/api/myob-business-api/v2/sale/credit_settlement"
); // /{cf_uri}/Sale/CreditSettlement
codegen::generate_model!(
    "CreditRefund",
    "https://developer.myob.com/api/myob-business-api/v2/sale/credit_refund"
); // /{cf_uri}/Sale/CreditRefund

codegen::generate_model!(
    "SaleInvoice",
    "https://developer.myob.com/api/myob-business-api/v2/sale/invoice"
); // /{cf_uri}/Sale/Invoice
codegen::generate_model!(
    "SaleInvoiceItem",
    "https://developer.myob.com/api/myob-business-api/v2/sale/invoice/invoice_item"
); // /{cf_uri}/Sale/Invoice/Item
codegen::generate_model!(
    "SaleInvoiceItemEmail",
    "https://developer.myob.com/api/myob-business-api/v2/sale/invoice/invoice_item/item-invoice-email"
); // /{cf_uri}/Sale/Invoice// /{cf_uri}/Sale/Invoice/Item/{invoice_uid}/Email
codegen::generate_model!(
    "SaleInvoiceService",
    "https://developer.myob.com/api/myob-business-api/v2/sale/invoice/invoice_service"
); // /{cf_uri}/Sale/Invoice/Service
codegen::generate_model!(
    "SaleInvoiceServiceEmail",
    "https://developer.myob.com/api/myob-business-api/v2/sale/invoice/invoice_service/service-invoice-email"
); // /{cf_uri}/Sale/Invoice/Service/{invoice_uid}/Email
codegen::generate_model!(
    "SaleInvoiceProfessional",
    "https://developer.myob.com/api/myob-business-api/v2/sale/invoice/invoice_professional"
); // /{cf_uri}/Sale/Invoice/Professional
codegen::generate_model!(
    "SaleInvoiceProfessionalEmail",
    "https://developer.myob.com/api/myob-business-api/v2/sale/invoice/invoice_professional/professional-invoice-email"
); // /{cf_uri}/Sale/Invoice/Professional/{invoice_uid}/Email
codegen::generate_model!(
    "SaleInvoiceTimeBilling",
    "https://developer.myob.com/api/myob-business-api/v2/sale/invoice/invoice_timebilling"
); // /{cf_uri}/Sale/Invoice/TimeBilling
codegen::generate_model!(
    "SaleInvoiceTimeBillingEmail",
    "https://developer.myob.com/api/myob-business-api/v2/sale/invoice/invoice_timebilling/timebilling-invoice-email"
); // /{cf_uri}/Sale/Invoice/TimeBilling/{invoice_uid}/Email
codegen::generate_model!(
    "SaleInvoiceMiscellaneous",
    "https://developer.myob.com/api/myob-business-api/v2/sale/invoice/invoice_miscellaneous"
); // /{cf_uri}/Sale/Invoice/Miscellaneous
codegen::generate_model!(
    "SaleInvoiceMiscellaneousEmail",
    "https://developer.myob.com/api/myob-business-api/v2/sale/invoice/invoice_miscellaneous/miscellaneous-invoice-email"
); // /{cf_uri}/Sale/Invoice/Miscellaneous/{invoice_uid}/Email

codegen::generate_model!(
    "SaleOrder",
    "https://developer.myob.com/api/myob-business-api/v2/sale/order"
); // /{cf_uri}/Sale/Order
codegen::generate_model!(
    "SaleOrderItem",
    "https://developer.myob.com/api/myob-business-api/v2/sale/order/order_item"
); // /{cf_uri}/Sale/Order/Item
codegen::generate_model!(
    "SaleOrderMiscellaneous",
    "https://developer.myob.com/api/myob-business-api/v2/sale/order/order_miscellaneous"
); // /{cf_uri}/Sale/Order/Miscellaneous
codegen::generate_model!(
    "SaleOrderProfessional",
    "https://developer.myob.com/api/myob-business-api/v2/sale/order/order_professional"
); // /{cf_uri}/Sale/Order/Professional
codegen::generate_model!(
    "SaleOrderService",
    "https://developer.myob.com/api/myob-business-api/v2/sale/order/order_service"
); // /{cf_uri}/Sale/Order/Service
codegen::generate_model!(
    "SaleOrderTimeBilling",
    "https://developer.myob.com/api/myob-business-api/v2/sale/order/order_timebilling"
); // /{cf_uri}/Sale/Order/TimeBilling

codegen::generate_model!(
    "SaleQuote",
    "https://developer.myob.com/api/myob-business-api/v2/sale/quote"
); // /{cf_uri}/Sale/Quote
codegen::generate_model!(
    "SaleQuoteItem",
    "https://developer.myob.com/api/myob-business-api/v2/sale/quote/quote_item"
); // /{cf_uri}/Sale/Quote/Item
codegen::generate_model!(
    "SaleQuoteMiscellaneous",
    "https://developer.myob.com/api/myob-business-api/v2/sale/quote/quote_miscellaneous"
); // /{cf_uri}/Sale/Quote/Miscellaneous
codegen::generate_model!(
    "SaleQuoteProfessional",
    "https://developer.myob.com/api/myob-business-api/v2/sale/quote/quote_professional"
); // /{cf_uri}/Sale/Quote/Professional
codegen::generate_model!(
    "SaleQuoteService",
    "https://developer.myob.com/api/myob-business-api/v2/sale/quote/quotes_service"
); // /{cf_uri}/Sale/Quote/Service
codegen::generate_model!(
    "SaleQuoteTimeBilling",
    "https://developer.myob.com/api/myob-business-api/v2/sale/quote/quote_timebilling"
); // /{cf_uri}/Sale/Quote/TimeBilling

// Purchase
codegen::generate_model!(
    "SupplierPayment",
    "https://developer.myob.com/api/myob-business-api/v2/purchase/supplierpayment"
); // /{cf_uri}/Purchase/SupplierPayment

codegen::generate_model!(
    "SupplierPaymentCalculateDiscounts",
    "https://developer.myob.com/api/myob-business-api/v2/purchase/supplierpayment/calculate_discounts"
); // /{cf_uri}/Purchase/SupplierPayment/CalculateDiscounts
codegen::generate_model!(
    "SupplierPaymentRecordWithDiscountsAndFees",
    "https://developer.myob.com/api/myob-business-api/v2/purchase/supplierpayment/record_with_discounts_fees"
); // /{cf_uri}/Purchase/SupplierPayment/RecordWithDiscountsAndFees

codegen::generate_model!(
    "DebitRefund",
    "https://developer.myob.com/api/myob-business-api/v2/purchase/debit_refund"
); // /{cf_uri}/Purchase/DebitRefund
codegen::generate_model!(
    "DebitSettlement",
    "https://developer.myob.com/api/myob-business-api/v2/purchase/debit_settlement"
); // /{cf_uri}/Purchase/DebitSettlement

codegen::generate_model!(
    "PurchaseBill",
    "https://developer.myob.com/api/myob-business-api/v2/purchase/bill"
); // /{cf_uri}/Purchase/Bill
codegen::generate_model!(
    "PurchaseBillItem",
    "https://developer.myob.com/api/myob-business-api/v2/purchase/bill/bill_item"
); // /{cf_uri}/Purchase/Bill/Item
codegen::generate_model!(
    "PurchaseBillItemAttachment",
    "https://developer.myob.com/api/myob-business-api/v2/purchase/bill/bill_item/item-bill-attachment"
); // /{cf_uri}/Purchase/Bill/Item/{Bill_UID}/Attachment
codegen::generate_model!(
    "PurchaseBillService",
    "https://developer.myob.com/api/myob-business-api/v2/purchase/bill/bill_service"
);
codegen::generate_model!(
    "PurchaseBillServiceAttachment",
    "https://developer.myob.com/api/myob-business-api/v2/purchase/bill/bill_service/service-bill-attachment"
); // /{cf_uri}/Purchase/Bill/Service/{Bill_UID}/Attachment
codegen::generate_model!(
    "PurchaseBillMiscellaneous",
    "https://developer.myob.com/api/myob-business-api/v2/purchase/bill/bill_miscellaneous"
); // /{cf_uri}/Purchase/Bill/Miscellaneous
codegen::generate_model!(
    "PurchaseBillMiscellaneousAttachment",
    "https://developer.myob.com/api/myob-business-api/v2/purchase/bill/bill_miscellaneous/miscellaneous-bill-attachment"
); // /{cf_uri}/Purchase/Bill/Miscellaneous/{Bill_UID}/Attachment
codegen::generate_model!(
    "PurchaseBillProfessional",
    "https://developer.myob.com/api/myob-business-api/v2/purchase/bill/bill_professional"
); // /{cf_uri}/Purchase/Bill/Professional/{Bill_UID}/Attachment
codegen::generate_model!(
    "PurchaseBillProfessionalAttachment",
    "https://developer.myob.com/api/myob-business-api/v2/purchase/bill/bill_professional/professional-bill-attachment"
); // /{cf_uri}/Purchase/Bill/Professional/{Bill_UID}/Attachment

codegen::generate_model!(
    "PurchaseOrder",
    "https://developer.myob.com/api/myob-business-api/v2/purchase/order"
); // /{cf_uri}/Purchase/Order
codegen::generate_model!(
    "PurchaseOrderItem",
    "https://developer.myob.com/api/myob-business-api/v2/purchase/order/order_item"
); // /{cf_uri}/Purchase/Order/Item
codegen::generate_model!(
    "PurchaseOrderMiscellaneous",
    "https://developer.myob.com/api/myob-business-api/v2/purchase/order/order_miscellaneous"
); // /{cf_uri}/Purchase/Order/Miscellaneous
codegen::generate_model!(
    "PurchaseOrderProfessional",
    "https://developer.myob.com/api/myob-business-api/v2/purchase/order/order_professional"
); // /{cf_uri}/Purchase/Order/Professional
codegen::generate_model!(
    "PurchaseOrderService",
    "https://developer.myob.com/api/myob-business-api/v2/purchase/order/order_service"
); // /{cf_uri}/Purchase/Order/Service

// Banking
codegen::generate_model!(
    "SpendMoneyTransaction",
    "https://developer.myob.com/api/myob-business-api/v2/banking/spend_money"
); // /{cf_uri}/Banking/SpendMoneyTxn
codegen::generate_model!(
    "SpendMoneyTransactionAttachments",
    "https://developer.myob.com/api/myob-business-api/v2/banking/spend_money/spend-money-attachments"
); // /{cf_uri}/Banking/SpendMoneyTxn/{Spend_Money_UID}/Attachment
codegen::generate_model!(
    "ReceiveMoneyTransaction",
    "https://developer.myob.com/api/myob-business-api/v2/banking/receive_money"
); // /{cf_uri}/Banking/ReceiveMoneyTxn
codegen::generate_model!(
    "TransferMoneyTransaction",
    "https://developer.myob.com/api/myob-business-api/v2/banking/transfer_money"
); // /{cf_uri}/Banking/TransferMoneyTxn
codegen::generate_model!(
    "Statement",
    "https://developer.myob.com/api/myob-business-api/v2/banking/statement"
); // /{cf_uri}/Banking/Statement
codegen::generate_model!(
    "BankAccount",
    "https://developer.myob.com/api/myob-business-api/v2/banking/bank-account"
); // /{cf_uri}/Banking/BankAccount

// Payroll
codegen::generate_model!(
    "PayrollCategory",
    "https://developer.myob.com/api/myob-business-api/v2/payroll/payroll-category"
); // /{cf_uri}/Payroll/PayrollCategory
codegen::generate_model!(
    "PayrollCategoryWage",
    "https://developer.myob.com/api/myob-business-api/v2/payroll/payroll-category/wage"
); // /{cf_uri}/Payroll/PayrollCategory/Wage
codegen::generate_model!(
    "PayrollCategoryEntitlement",
    "https://developer.myob.com/api/myob-business-api/v2/payroll/payroll-category/entitlement"
); // /{cf_uri}/Payroll/PayrollCategory/Entitlement
codegen::generate_model!(
    "PayrollCategoryDeduction",
    "https://developer.myob.com/api/myob-business-api/v2/payroll/payroll-category/deduction"
); // /{cf_uri}/Payroll/PayrollCategory/Deduction
codegen::generate_model!(
    "PayrollCategoryExpense",
    "https://developer.myob.com/api/myob-business-api/v2/payroll/payroll-category/expense"
); // /{cf_uri}/Payroll/PayrollCategory/Expense
codegen::generate_model!(
    "PayrollCategorySuperannuation",
    "https://developer.myob.com/api/myob-business-api/v2/payroll/payroll-category/superannuation"
); // /{cf_uri}/Payroll/PayrollCategory/Superannuation
codegen::generate_model!(
    "PayrollCategoryTax",
    "https://developer.myob.com/api/myob-business-api/v2/payroll/payroll-category/tax"
); // /{cf_uri}/Payroll/PayrollCategory/Tax
codegen::generate_model!(
    "PayrollCategoryTaxTable",
    "https://developer.myob.com/api/myob-business-api/v2/payroll/payroll-category/tax-table"
); // /{cf_uri}/Payroll/PayrollCategory/TaxTable

codegen::generate_model!(
    "SuperannuationFund",
    "https://developer.myob.com/api/myob-business-api/v2/payroll/superannuationfund"
); // /{cf_uri}/Payroll/SuperannuationFund
codegen::generate_model!(
    "EmploymentClassification",
    "https://developer.myob.com/api/myob-business-api/v2/payroll/employment-classification"
); // /{cf_uri}/Payroll/EmploymentClassification
codegen::generate_model!(
    "Timesheet",
    "https://developer.myob.com/api/myob-business-api/v2/payroll/timesheet"
); // /{cf_uri}/Payroll/Timesheet

// Timebilling
codegen::generate_model!(
    "TimeBillingActivity",
    "https://developer.myob.com/api/myob-business-api/v2/timebilling/activity"
); // /{cf_uri}/TimeBilling/Activity
codegen::generate_model!(
    "TimeBillingActivitySlip",
    "https://developer.myob.com/api/myob-business-api/v2/timebilling/activity-slip"
); // /{cf_uri}/TimeBilling/ActivitySlip

// Company
codegen::generate_model!(
    "CompanyCustomList",
    "https://developer.myob.com/api/myob-business-api/v2/company/custom-list"
); // /{cf_uri}/Company/CustomList
codegen::generate_model!(
    "CompanyFormTemplate",
    "https://developer.myob.com/api/myob-business-api/v2/company/form_template"
); // /{cf_uri}/Company/FormTemplate
codegen::generate_model!(
    "CompanyPreferences",
    "https://developer.myob.com/api/myob-business-api/v2/company/preferences"
); // /{cf_uri}/Company/Preferences
