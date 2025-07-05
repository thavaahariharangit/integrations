/*
    Copyright (C) 2017-2025 Yuriy Yarosh.
    All rights reserved.
*/

use bitsnap_accounting_stripe_codegen as codegen;

// Core
codegen::generate_model!("Balance", "https://docs.stripe.com/api/balance");
codegen::generate_model!("BalanceTransactions", "https://docs.stripe.com/api/balance_transactions");
codegen::generate_model!("Charges", "https://docs.stripe.com/api/charges");
codegen::generate_model!("Customers", "https://docs.stripe.com/api/customers");
codegen::generate_model!("CustomerSessions", "https://docs.stripe.com/api/customer_sessions");
codegen::generate_model!("Disputes", "https://docs.stripe.com/api/disputes");
codegen::generate_model!("Events", "https://docs.stripe.com/api/events");

codegen::generate_model!("Events", "https://docs.stripe.com/api/v2/core/events");
codegen::generate_model!("EventDestinations", "https://docs.stripe.com/api/v2/core/event_destinations");

codegen::generate_model!("Files", "https://docs.stripe.com/api/files");
codegen::generate_model!("FileLinks", "https://docs.stripe.com/api/file_links");
codegen::generate_model!("Mandates", "https://docs.stripe.com/api/mandates");
codegen::generate_model!("PaymentIntents", "https://docs.stripe.com/api/payment_intents");
codegen::generate_model!("SetupIntents", "https://docs.stripe.com/api/setup_intents");
codegen::generate_model!("SetupAttempts", "https://docs.stripe.com/api/setup_attempts");
codegen::generate_model!("Payouts", "https://docs.stripe.com/api/payouts");
codegen::generate_model!("Payouts", "https://docs.stripe.com/api/refunds");
codegen::generate_model!("Payouts", "https://docs.stripe.com/api/confirmation_tokens");
codegen::generate_model!("Payouts", "https://docs.stripe.com/api/tokens");

// Payment Methods 
codegen::generate_model!("PaymentMethods", "https://docs.stripe.com/api/payment_methods");
codegen::generate_model!("PaymentMethodConfigurations", "https://docs.stripe.com/api/payment_method_configurations");
codegen::generate_model!("PaymentMethodDomains", "https://docs.stripe.com/api/payment_method_domains");
codegen::generate_model!("CustomerBankAccounts", "https://docs.stripe.com/api/customer_bank_accounts");
codegen::generate_model!("CashBalance", "https://docs.stripe.com/api/cash_balance");
codegen::generate_model!("CashBalanceTransactions", "https://docs.stripe.com/api/cash_balance_transactions");
codegen::generate_model!("Cards", "https://docs.stripe.com/api/cards");
codegen::generate_model!("Sources", "https://docs.stripe.com/api/sources"); // Deprecated

// Products
codegen::generate_model!("Products", "https://docs.stripe.com/api/products");
codegen::generate_model!("Prices", "https://docs.stripe.com/api/prices");
codegen::generate_model!("Coupons", "https://docs.stripe.com/api/coupons");
codegen::generate_model!("PromotionCodes", "https://docs.stripe.com/api/promotion_codes");
codegen::generate_model!("Discounts", "https://docs.stripe.com/api/discounts");
codegen::generate_model!("TaxCodes", "https://docs.stripe.com/api/tax_codes");
codegen::generate_model!("TaxRates", "https://docs.stripe.com/api/tax_rates");
codegen::generate_model!("ShippingRates", "https://docs.stripe.com/api/shipping_rates");

// Checkout Sessions
codegen::generate_model!("CheckoutSession", "https://docs.stripe.com/api/checkout/sessions");


// Payment Link
codegen::generate_model!("PaymentLink", "https://docs.stripe.com/api/payment-link");

// Billing
codegen::generate_model!("CreditNotes", "https://docs.stripe.com/api/credit_notes");
codegen::generate_model!("CustomerBalanceTransactions", "https://docs.stripe.com/api/customer_balance_transactions");
codegen::generate_model!("CustomerPortalSessions", "https://docs.stripe.com/api/customer_portal/sessions");
codegen::generate_model!("CustomerPortalConfigurations", "https://docs.stripe.com/api/customer_portal/configurations");
codegen::generate_model!("Invoices", "https://docs.stripe.com/api/invoices");
codegen::generate_model!("InvoiceItems", "https://docs.stripe.com/api/invoiceitems");
codegen::generate_model!("InvoiceLineItem", "https://docs.stripe.com/api/invoice-line-item");
codegen::generate_model!("InvoiceRenderingTemplate", "https://docs.stripe.com/api/invoice-rendering-template");

codegen::generate_model!("BillingAlert", "https://docs.stripe.com/api/billing/alert");
codegen::generate_model!("BillingMeter", "https://docs.stripe.com/api/billing/meter");
codegen::generate_model!("BillingMeterEvent", "https://docs.stripe.com/api/billing/meter-event");
codegen::generate_model!("BillingMeterEventAdjustment", "https://docs.stripe.com/api/billing/meter-event-adjustment");
codegen::generate_model!("BillingMeterEventSummary", "https://docs.stripe.com/api/billing/meter-event-summary");

codegen::generate_model!("BillingMeterV2", "https://docs.stripe.com/api/v2/billing-meter");
codegen::generate_model!("BillingMeterEventAdjustmentV2", "https://docs.stripe.com/api/v2/billing-meter-adjustment");
codegen::generate_model!("BillingMeterStreamV2", "https://docs.stripe.com/api/v2/billing-meter-stream");


codegen::generate_model!("CreditGrant", "https://docs.stripe.com/api/billing/credit-grant");
codegen::generate_model!("CreditBalanceSummary", "https://docs.stripe.com/api/billing/credit-balance-summary");
codegen::generate_model!("CreditBalanceTransaction", "https://docs.stripe.com/api/billing/credit-balance-transaction");

codegen::generate_model!("Plans", "https://docs.stripe.com/api/plans");
codegen::generate_model!("Quotes", "https://docs.stripe.com/api/quotes");

codegen::generate_model!("Subscriptions", "https://docs.stripe.com/api/subscriptions");
codegen::generate_model!("SubscriptionItems", "https://docs.stripe.com/api/subscription_items");
codegen::generate_model!("SubscriptionSchedules", "https://docs.stripe.com/api/subscription_schedules");

codegen::generate_model!("TaxIds", "https://docs.stripe.com/api/tax_ids");
codegen::generate_model!("TestClocks", "https://docs.stripe.com/api/test_clocks"); // TEST HELPER

codegen::generate_model!("UsageRecords", "https://docs.stripe.com/api/usage_records"); 
codegen::generate_model!("UsageRecordSummary", "https://docs.stripe.com/api/usage-record-summary");

// Capital
codegen::generate_model!("CapitalFinancingOffers", "https://docs.stripe.com/api/capital/financing_offers");
codegen::generate_model!("CapitalFinancingSummary", "https://docs.stripe.com/api/capital/financing_summary");

// Connect
codegen::generate_model!("ConnectAccounts", "https://docs.stripe.com/api/accounts");
codegen::generate_model!("ConnectLoginLink", "https://docs.stripe.com/api/accounts/login_link");
codegen::generate_model!("ConnectAccountLinks", "https://docs.stripe.com/api/account_links");
codegen::generate_model!("ConnectAccountSessions", "https://docs.stripe.com/api/account_sessions");
codegen::generate_model!("ConnectApplicationFees", "https://docs.stripe.com/api/application_fees");
codegen::generate_model!("ConnectFeeRefunds", "https://docs.stripe.com/api/fee_refunds");
codegen::generate_model!("ConnectCapabilities", "https://docs.stripe.com/api/capabilities");
codegen::generate_model!("ConnectCountrySpecs", "https://docs.stripe.com/api/country_specs");
codegen::generate_model!("ConnectExternalAccounts", "https://docs.stripe.com/api/external_accounts");
codegen::generate_model!("ConnectExternalAccountCards", "https://docs.stripe.com/api/external_account_cards");
codegen::generate_model!("ConnectPersons", "https://docs.stripe.com/api/persons");
codegen::generate_model!("ConnectTopups", "https://docs.stripe.com/api/topups");
codegen::generate_model!("ConnectTransfers", "https://docs.stripe.com/api/transfers");
codegen::generate_model!("ConnectTransferReversals", "https://docs.stripe.com/api/transfer_reversals");
codegen::generate_model!("ConnectSecretManagement", "https://docs.stripe.com/api/secret_management");

// Fraud
codegen::generate_model!("EarlyFraudWarnings", "https://docs.stripe.com/api/radar/early_fraud_warnings");
codegen::generate_model!("FraudReviews", "https://docs.stripe.com/api/radar/reviews");
codegen::generate_model!("FraudValueLists", "https://docs.stripe.com/api/radar/value_lists");
codegen::generate_model!("FraudValueListItems", "https://docs.stripe.com/api/radar/value_list_items");

// Issuing
codegen::generate_model!("IssuingAuthorizations", "https://docs.stripe.com/api/issuing/authorizations");
codegen::generate_model!("IssuingCardHolders", "https://docs.stripe.com/api/issuing/cardholders");
codegen::generate_model!("IssuingCards", "https://docs.stripe.com/api/issuing/cards");
codegen::generate_model!("IssuingDisputes", "https://docs.stripe.com/api/issuing/disputes");
codegen::generate_model!("IssuingFundingInstructions", "https://docs.stripe.com/api/issuing/funding_instructions");
codegen::generate_model!("IssuingPersonalizationDesigns", "https://docs.stripe.com/api/issuing/personalization_designs");
codegen::generate_model!("IssuingPhysicalBundles", "https://docs.stripe.com/api/issuing/physical_bundles");
codegen::generate_model!("IssuingTokens", "https://docs.stripe.com/api/issuing/tokens");
codegen::generate_model!("IssuingTransactions", "https://docs.stripe.com/api/issuing/transactions");

// Terminal
codegen::generate_model!("TerminalConnectionTokens", "https://docs.stripe.com/api/terminal/connection_tokens");
codegen::generate_model!("TerminalLocations", "https://docs.stripe.com/api/terminal/locations");
codegen::generate_model!("TerminalReaders", "https://docs.stripe.com/api/terminal/readers");
codegen::generate_model!("TerminalHardwareOrders", "https://docs.stripe.com/api/terminal/hardware_orders");
codegen::generate_model!("TerminalHardwareProducts", "https://docs.stripe.com/api/terminal/hardware_products");
codegen::generate_model!("TerminalHardwareSkus", "https://docs.stripe.com/api/terminal/hardware_skus");
codegen::generate_model!("TerminalHardwareShippingMethods", "https://docs.stripe.com/api/terminal/hardware_shipping_methods");
codegen::generate_model!("TerminalConfiguration", "https://docs.stripe.com/api/terminal/configuration");

// Treasury
codegen::generate_model!("TreasuryFinancialAccounts", "https://docs.stripe.com/api/treasury/financial_accounts");
codegen::generate_model!("TreasuryFinancialAccountFeatures", "https://docs.stripe.com/api/treasury/financial_account_features");
codegen::generate_model!("TreasuryTransactions", "https://docs.stripe.com/api/treasury/transactions");
codegen::generate_model!("TreasuryTransactionEntries", "https://docs.stripe.com/api/treasury/transaction_entries");
codegen::generate_model!("TreasuryOutboundTransfers", "https://docs.stripe.com/api/treasury/outbound_transfers");
codegen::generate_model!("TreasuryOutboundPayments", "https://docs.stripe.com/api/treasury/outbound_payments");
codegen::generate_model!("TreasuryInboundTransfers", "https://docs.stripe.com/api/treasury/inbound_transfers");
codegen::generate_model!("TreasuryReceivedCredits", "https://docs.stripe.com/api/treasury/received_credits");
codegen::generate_model!("TreasuryReceivedDebits", "https://docs.stripe.com/api/treasury/received_debits");
codegen::generate_model!("TreasuryCreditReversals", "https://docs.stripe.com/api/treasury/credit_reversals");
codegen::generate_model!("TreasuryDebitReversals", "https://docs.stripe.com/api/treasury/debit_reversals");

// entitlements
codegen::generate_model!("EntitlementFeature", "https://docs.stripe.com/api/entitlements/feature");
codegen::generate_model!("EntitlementProductFeature", "https://docs.stripe.com/api/product-feature");
codegen::generate_model!("EntitlementActiveEntitlement", "https://docs.stripe.com/api/entitlements/active-entitlement");

// Sigma
codegen::generate_model!("SigmaScheduledQueries", "https://docs.stripe.com/api/sigma/scheduled_queries");

// Reporting
codegen::generate_model!("ReportingReportRun", "https://docs.stripe.com/api/reporting/report_run");
codegen::generate_model!("ReportingReportType", "https://docs.stripe.com/api/reporting/report_type");

// Financial connections
codegen::generate_model!("FinancialAccounts", "https://docs.stripe.com/api/financial_connections/accounts");
codegen::generate_model!("FinancialOwnership", "https://docs.stripe.com/api/financial_connections/ownership");
codegen::generate_model!("FinancialSessions", "https://docs.stripe.com/api/financial_connections/sessions");
codegen::generate_model!("FinancialTransactions", "https://docs.stripe.com/api/financial_connections/transactions");

// Tax
codegen::generate_model!("TaxCalculations", "https://docs.stripe.com/api/tax/calculations");
codegen::generate_model!("TaxRegistrations", "https://docs.stripe.com/api/tax/registrations");
codegen::generate_model!("TaxTransactions", "https://docs.stripe.com/api/tax/transactions");
codegen::generate_model!("TaxSettings", "https://docs.stripe.com/api/tax/settings");

// Identity
codegen::generate_model!("IdentityVerificationSessions", "https://docs.stripe.com/api/identity/verification_sessions");
codegen::generate_model!("IdentityVerificationReports", "https://docs.stripe.com/api/identity/verification_reports");

// Crypto
codegen::generate_model!("CryptoOnrampSessions", "https://docs.stripe.com/api/crypto/onramp_sessions");
codegen::generate_model!("CryptoOnrampQuotes", "https://docs.stripe.com/api/crypto/onramp_quotes");

// Climate
codegen::generate_model!("ClimateOrder", "https://docs.stripe.com/api/climate/order");
codegen::generate_model!("ClimateProduct", "https://docs.stripe.com/api/climate/product");
codegen::generate_model!("ClimateSupplier", "https://docs.stripe.com/api/climate/supplier");

// Forwarding
codegen::generate_model!("ForwardingRequest", "https://docs.stripe.com/api/forwarding/request");

// Webhook Endpoints
codegen::generate_model!("WebhookEndpoints", "https://docs.stripe.com/api/webhook_endpoints");
