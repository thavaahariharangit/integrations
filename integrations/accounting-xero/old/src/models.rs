/*
    Copyright (C) 2017-2025 Yuriy Yarosh.
    All rights reserved.
*/

use bitsnap_accounting_xero_codegen as codegen;

// Accounting

codegen::generate_model!("Account", "https://developer.xero.com/documentation/api/accounting/accounts");
codegen::generate_model!("Attachment", "https://developer.xero.com/documentation/api/accounting/attachments");
codegen::generate_model!("BankStatement", "https://developer.xero.com/documentation/api/accounting/bankstatements");
codegen::generate_model!("BankTransaction", "https://developer.xero.com/documentation/api/accounting/banktransactions");
codegen::generate_model!("BankTransfer", "https://developer.xero.com/documentation/api/accounting/banktransfers");


codegen::generate_model!("BatchPayment", "https://developer.xero.com/documentation/api/accounting/batchpayments");
codegen::generate_model!("BrandingTheme", "https://developer.xero.com/documentation/api/accounting/brandingthemes");
codegen::generate_model!("Budget", "https://developer.xero.com/documentation/api/accounting/budgets");
codegen::generate_model!("ContactGroup", "https://developer.xero.com/documentation/api/accounting/contactgroups");
codegen::generate_model!("Contact", "https://developer.xero.com/documentation/api/accounting/contacts");
codegen::generate_model!("CreditNote", "https://developer.xero.com/documentation/api/accounting/creditnotes");
codegen::generate_model!("Currency", "https://developer.xero.com/documentation/api/accounting/currencies");
codegen::generate_model!("Employee", "https://developer.xero.com/documentation/api/accounting/employees");
codegen::generate_model!("ExpenseClaim", "https://developer.xero.com/documentation/api/accounting/expenseclaims"); // Deprecated
codegen::generate_model!("RequestsAndResponses", "https://developer.xero.com/documentation/api/accounting/requests-and-responses"); // Deprecated
codegen::generate_model!("HistoryAndNotes", "https://developer.xero.com/documentation/api/accounting/historyandnotes");
codegen::generate_model!("InvoiceReminder", "https://developer.xero.com/documentation/api/accounting/invoicereminders");
codegen::generate_model!("Invoice", "https://developer.xero.com/documentation/api/accounting/invoices");
codegen::generate_model!("Item", "https://developer.xero.com/documentation/api/accounting/items");
codegen::generate_model!("Journal", "https://developer.xero.com/documentation/api/accounting/journals");
codegen::generate_model!("LinkedTransaction", "https://developer.xero.com/documentation/api/accounting/linkedtransactions");
codegen::generate_model!("ManualJournal", "https://developer.xero.com/documentation/api/accounting/manualjournals");
codegen::generate_model!("Organisation", "https://developer.xero.com/documentation/api/accounting/organisation");
codegen::generate_model!("Overpayment", "https://developer.xero.com/documentation/api/accounting/overpayments");
codegen::generate_model!("PaymentService", "https://developer.xero.com/documentation/api/accounting/paymentservices");
codegen::generate_model!("Payment", "https://developer.xero.com/documentation/api/accounting/payments");
codegen::generate_model!("PrePayment", "https://developer.xero.com/documentation/api/accounting/prepayments");
codegen::generate_model!("PurchaseOrder", "https://developer.xero.com/documentation/api/accounting/purchaseorders");
codegen::generate_model!("Quote", "https://developer.xero.com/documentation/api/accounting/quotes");
codegen::generate_model!("Receipt", "https://developer.xero.com/documentation/api/accounting/receipts"); // Deprecated
codegen::generate_model!("ReleaseNote", "https://developer.xero.com/documentation/api/accounting/releasenotes");
codegen::generate_model!("RepeatingInvoice", "https://developer.xero.com/documentation/api/accounting/repeatinginvoices");
codegen::generate_model!("Report", "https://developer.xero.com/documentation/api/accounting/reports");
codegen::generate_model!("ResponseCode", "https://developer.xero.com/documentation/api/accounting/responsecodes");
codegen::generate_model!("TaxRate", "https://developer.xero.com/documentation/api/accounting/taxrates");
codegen::generate_model!("TrackingCategory", "https://developer.xero.com/documentation/api/accounting/trackingcategories");
codegen::generate_model!("User", "https://developer.xero.com/documentation/api/accounting/users");

// Accounting enums
// https://developer.xero.com/documentation/api/accounting/types

// Assets API

codegen::generate_model!("AssetType", "https://developer.xero.com/documentation/api/assets/assettypes");
codegen::generate_model!("Asset", "https://developer.xero.com/documentation/api/assets/assets");
codegen::generate_model!("AssetSetting", "https://developer.xero.com/documentation/api/assets/settings");

// Assets enums
// https://developer.xero.com/documentation/api/assets/types

// Files API

codegen::generate_model!("FileAssociation", "https://developer.xero.com/documentation/api/files/associations");

codegen::generate_model!("File", "https://developer.xero.com/documentation/api/files/files");
codegen::generate_model!("FileFolder", "https://developer.xero.com/documentation/api/files/folders");

// Files enums
// https://developer.xero.com/documentation/api/files/types

// Finance API

codegen::generate_model!("AccountingActivity", "https://developer.xero.com/documentation/api/finance/accountingactivities");
codegen::generate_model!("BankStatementPlus", "https://developer.xero.com/documentation/api/finance/bankstatementsplus");

codegen::generate_model!("CashValidation", "https://developer.xero.com/documentation/api/finance/cashvalidation");
codegen::generate_model!("FinancialStatement", "https://developer.xero.com/documentation/api/finance/financialstatements");

// Payroll AU

codegen::generate_model!("Employee", "https://developer.xero.com/documentation/api/payrollau/employees");
codegen::generate_model!("LeaveApplication", "https://developer.xero.com/documentation/api/payrollau/leaveapplications");
codegen::generate_model!("LeaveBalance", "https://developer.xero.com/documentation/api/payrollau/leavebalances");
codegen::generate_model!("PayItem", "https://developer.xero.com/documentation/api/payrollau/payitems");
codegen::generate_model!("PayRun", "https://developer.xero.com/documentation/api/payrollau/payruns");
codegen::generate_model!("PayrollCalendar", "https://developer.xero.com/documentation/api/payrollau/payrollcalendars");
codegen::generate_model!("PaySlip", "https://developer.xero.com/documentation/api/payrollau/payslip");
codegen::generate_model!("STPChange", "https://developer.xero.com/documentation/api/payrollau/stp-changes");
codegen::generate_model!("Setting", "https://developer.xero.com/documentation/api/payrollau/settings");
codegen::generate_model!("SuperFundProduct", "https://developer.xero.com/documentation/api/payrollau/superfundproducts");
codegen::generate_model!("SuperFund", "https://developer.xero.com/documentation/api/payrollau/superfunds");
codegen::generate_model!("Timesheet", "https://developer.xero.com/documentation/api/payrollau/timesheets");

// ENUMS
// https://developer.xero.com/documentation/api/payrollau/types-and-codes

// Payroll NZ

codegen::generate_model!("Deduction", "https://developer.xero.com/documentation/api/payrollnz/deductions");
codegen::generate_model!("EarningsRate", "https://developer.xero.com/documentation/api/payrollnz/earningsrates");
codegen::generate_model!("EmployeeWorkingPattern", "https://developer.xero.com/documentation/api/payrollnz/employeeworkingpatterns");
codegen::generate_model!("Employee", "https://developer.xero.com/documentation/api/payrollnz/employees");
codegen::generate_model!("EmployeeLeave", "https://developer.xero.com/documentation/api/payrollnz/employeeleave");
codegen::generate_model!("EmployeeLeavePeriod", "https://developer.xero.com/documentation/api/payrollnz/employeeleaveperiods");
codegen::generate_model!("EmployeeLeaveSetup", "https://developer.xero.com/documentation/api/payrollnz/employeeleavesetup");
codegen::generate_model!("EmployeeLeaveType", "https://developer.xero.com/documentation/api/payrollnz/employeeleavetypes");
codegen::generate_model!("EmployeeOpeningBalance", "https://developer.xero.com/documentation/api/payrollnz/employeeopeningbalances");
codegen::generate_model!("EmployeePayTemplate", "https://developer.xero.com/documentation/api/payrollnz/employeepaytemplates");
codegen::generate_model!("EmployeeTax", "https://developer.xero.com/documentation/api/payrollnz/employeetax");

codegen::generate_model!("Employment", "https://developer.xero.com/documentation/api/payrollnz/employment");
codegen::generate_model!("LeaveBalance", "https://developer.xero.com/documentation/api/payrollnz/leavebalances");

codegen::generate_model!("EmployeeLeaveType", "https://developer.xero.com/documentation/api/payrollnz/employeeleavetypes");
codegen::generate_model!("PayRunCalendar", "https://developer.xero.com/documentation/api/payrollnz/payruncalendars");
codegen::generate_model!("PaymentMethod", "https://developer.xero.com/documentation/api/payrollnz/paymentmethods");
codegen::generate_model!("PayRun", "https://developer.xero.com/documentation/api/payrollnz/payruns");
codegen::generate_model!("PaySlip", "https://developer.xero.com/documentation/api/payrollnz/payslips");
codegen::generate_model!("Reimbursement", "https://developer.xero.com/documentation/api/payrollnz/reimbursements");
codegen::generate_model!("SalaryAndWages", "https://developer.xero.com/documentation/api/payrollnz/salaryandwages");
codegen::generate_model!("Settings", "https://developer.xero.com/documentation/api/payrollnz/settings");
codegen::generate_model!("StatutoryDeduction", "https://developer.xero.com/documentation/api/payrollnz/statutorydeductions");
codegen::generate_model!("Superannuation", "https://developer.xero.com/documentation/api/payrollnz/superannuation");
codegen::generate_model!("Timesheet", "https://developer.xero.com/documentation/api/payrollnz/timesheets");
codegen::generate_model!("TrackingCategory", "https://developer.xero.com/documentation/api/payrollnz/trackingcategories");

// https://developer.xero.com/documentation/api/payrollnz/nztaxcodes
// https://developer.xero.com/documentation/api/payrollnz/typesandcodes

// Payroll UK

codegen::generate_model!("Deduction", "https://developer.xero.com/documentation/api/payrolluk/deductions");
codegen::generate_model!("EarningRate", "https://developer.xero.com/documentation/api/payrolluk/earningrates");
codegen::generate_model!("EarningsOrder", "https://developer.xero.com/documentation/api/payrolluk/earningsorders");
codegen::generate_model!("EmployeeOpeningBalance", "https://developer.xero.com/documentation/api/payrolluk/employeeopeningbalances");
codegen::generate_model!("EmployeeWorkingPattern", "https://developer.xero.com/documentation/api/payrolluk/employeeworkingpatterns");
codegen::generate_model!("Employee", "https://developer.xero.com/documentation/api/payrolluk/employees");
codegen::generate_model!("EmployeeLeave", "https://developer.xero.com/documentation/api/payrolluk/employeeleave");
codegen::generate_model!("EmployeeLeaveBalance", "https://developer.xero.com/documentation/api/payrolluk/employeeleavebalances");
codegen::generate_model!("EmployeeLeavePeriod", "https://developer.xero.com/documentation/api/payrolluk/employeeleaveperiods");
codegen::generate_model!("EmployeeLeaveType", "https://developer.xero.com/documentation/api/payrolluk/employeeleavetypes");
codegen::generate_model!("EmployeePayTemplate", "https://developer.xero.com/documentation/api/payrolluk/employeepaytemplates");
codegen::generate_model!("EmployeeStatutoryLeaveBalance", "https://developer.xero.com/documentation/api/payrolluk/employeestatutoryleavebalance");
codegen::generate_model!("EmployeeStatutoryLeaveSummary", "https://developer.xero.com/documentation/api/payrolluk/employeestatutoryleavessummary");
codegen::generate_model!("EmployeeStatutorySickLeave", "https://developer.xero.com/documentation/api/payrolluk/employeestatutorysickleave");
codegen::generate_model!("EmployeeTax", "https://developer.xero.com/documentation/api/payrolluk/employeetax");
codegen::generate_model!("EmployeePension", "https://developer.xero.com/documentation/api/payrolluk/employerpensions");
codegen::generate_model!("Employment", "https://developer.xero.com/documentation/api/payrolluk/employment");
codegen::generate_model!("LeaveType", "https://developer.xero.com/documentation/api/payrolluk/leavetypes");
codegen::generate_model!("PayRunCalendar", "https://developer.xero.com/documentation/api/payrolluk/payruncalendars");
codegen::generate_model!("PayRun", "https://developer.xero.com/documentation/api/payrolluk/payruns");
codegen::generate_model!("PaymentMethod", "https://developer.xero.com/documentation/api/payrolluk/paymentmethods");
codegen::generate_model!("PaySlip", "https://developer.xero.com/documentation/api/payrolluk/payslips");
codegen::generate_model!("Reimbursement", "https://developer.xero.com/documentation/api/payrolluk/reimbursements");
codegen::generate_model!("SalaryAndWages", "https://developer.xero.com/documentation/api/payrolluk/salaryandwages");
codegen::generate_model!("Settings", "https://developer.xero.com/documentation/api/payrolluk/settings");
codegen::generate_model!("Timesheet", "https://developer.xero.com/documentation/api/payrolluk/timesheets");
codegen::generate_model!("TrackingCategory", "https://developer.xero.com/documentation/api/payrolluk/trackingcategories");

// https://developer.xero.com/documentation/api/payrolluk/typesandcodes

// Projects

codegen::generate_model!("Project", "https://developer.xero.com/documentation/api/projects/projects");
codegen::generate_model!("Task", "https://developer.xero.com/documentation/api/projects/tasks");
codegen::generate_model!("Time", "https://developer.xero.com/documentation/api/projects/time");
codegen::generate_model!("User", "https://developer.xero.com/documentation/api/projects/users");
