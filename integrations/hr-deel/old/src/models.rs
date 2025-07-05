/*
    Copyright (C) 2017-2025 Yuriy Yarosh.
    All rights reserved.
*/

use bitsnap_hr_deel_codegen as codegen;

// REST API
codegen::generate_model!("Contracts", "https://developer.deel.com/reference/getcontractlist");
codegen::generate_model!("Groups", "https://developer.deel.com/reference/getgroups");
codegen::generate_model!("Policies", "https://developer.deel.com/reference/getpoliciesforprofile");
codegen::generate_model!("Countries", "https://developer.deel.com/reference/getcountries");
codegen::generate_model!("Currencies", "https://developer.deel.com/reference/getcurrencies");
codegen::generate_model!("JobTitles", "https://developer.deel.com/reference/getjobtitlelist");
codegen::generate_model!("Seniorities", "https://developer.deel.com/reference/getsenioritylist");
codegen::generate_model!("TimeOffTypes", "https://developer.deel.com/reference/gettimeofftypelist");
codegen::generate_model!("Lookups", "https://developer.deel.com/reference/getlookups");
codegen::generate_model!("LegalEntities", "https://developer.deel.com/reference/getlegalentitylist");
codegen::generate_model!("Organizations", "https://developer.deel.com/reference/getorganizations");
codegen::generate_model!("Teams", "https://developer.deel.com/reference/getteams");
codegen::generate_model!("Departments", "https://developer.deel.com/reference/getdepartments");
codegen::generate_model!("WorkingLocations", "https://developer.deel.com/reference/getworkinglocations");
codegen::generate_model!("Managers", "https://developer.deel.com/reference/getmanagers");
codegen::generate_model!("Adjustments", "https://developer.deel.com/reference/getadjustments");
codegen::generate_model!("Categories", "https://developer.deel.com/reference/getcategories");
codegen::generate_model!("Webhooks", "https://developer.deel.com/reference/getallwebhooks");
codegen::generate_model!("WebhookEventTypes", "https://developer.deel.com/reference/getallwebhookeventtypes");
codegen::generate_model!("DetailedPaymentsReport", "https://developer.deel.com/reference/getdetailedpaymentsreport");
codegen::generate_model!("BackgroundChecksOptions", "https://developer.deel.com/reference/getbackgroundchecksoptions");
codegen::generate_model!("BackgroundChecks", "https://developer.deel.com/reference/getbackgroundchecksbycontractid");


// codegen::generate_model!("ImmigrationVisaTypes", "https://developer.deel.com/reference/getimmigrationvisatypes");
// codegen::generate_model!("ImmigrationCaseDetails", "https://developer.deel.com/reference/immigrationcasedetails");
// codegen::generate_model!("ImmigrationDocument", "https://developer.deel.com/reference/immigrationdocument");
codegen::generate_model!("OffboardingTrackerId", "https://developer.deel.com/reference/get_offboarding-tracker-id");
codegen::generate_model!("OffboardingTrackerHrisProfileOid", "https://developer.deel.com/reference/get_offboarding-tracker-hris-profile-oid");
codegen::generate_model!("OffboardingTracker", "https://developer.deel.com/reference/get_offboarding-tracker");
codegen::generate_model!("VerificationMethod", "https://developer.deel.com/reference/getverificationmethod");

// Accounting
codegen::generate_model!("Invoice", "https://developer.deel.com/reference/getinvoicelist");
codegen::generate_model!("DeeInvoice", "https://developer.deel.com/reference/getdeelinvoicelist");
codegen::generate_model!("Payment", "https://developer.deel.com/reference/getpaymentlist");
codegen::generate_model!("PaymentsBreakdownById", "https://developer.deel.com/reference/getpaymentsbreakdownbyid");
codegen::generate_model!("BillingInvoiceDownloadLink", "https://developer.deel.com/reference/getbillinginvoicedownloadlink");


// HR SCIM
codegen::generate_model!("SearchVIAGet", "https://developer.deel.com/reference/searchviaget");
codegen::generate_model!("User", "https://developer.deel.com/reference/getuserbyid");


// HR
codegen::generate_model!("WorkerRelationTypes", "https://developer.deel.com/reference/getallworkerrelationtypes");
codegen::generate_model!("ProfileWorkerRelations", "https://developer.deel.com/reference/getallprofileworkerrelations");
codegen::generate_model!("CustomFieldValuesFromContract", "https://developer.deel.com/reference/getcustomfieldvaluesfromcontract");

// Payroll

codegen::generate_model!("BankAccounts", "https://developer.deel.com/reference/getgpbankaccounts");
codegen::generate_model!("BankGuide", "https://developer.deel.com/reference/getgpbankguide");
codegen::generate_model!("WorkerPayslips", "https://developer.deel.com/reference/getworkerpayslips");
codegen::generate_model!("DownloadPayslipPDF", "https://developer.deel.com/reference/getdownloadurlforgppayslip");
codegen::generate_model!("GrossToNetGPReport", "https://developer.deel.com/reference/getgrosstonetgpreports");
codegen::generate_model!("GPEntity", "https://developer.deel.com/reference/getgplegalentities");
codegen::generate_model!("Shifts", "https://developer.deel.com/reference/getshifts");
codegen::generate_model!("ShiftRates", "https://developer.deel.com/reference/getshiftrates");

