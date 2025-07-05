/*
    Copyright (C) 2017-2025 Yuriy Yarosh.
    All rights reserved.
*/

use bitsnap_accounting_paypal_codegen as codegen;

codegen::generate_model!("Tracking", "https://developer.paypal.com/docs/api/tracking/v1/");
codegen::generate_model!("CatalogProducts", "https://developer.paypal.com/docs/api/catalog-products/v1/");
codegen::generate_model!("CustomerDisputes", "https://developer.paypal.com/docs/api/customer-disputes/v1/");
codegen::generate_model!("Identity", "https://developer.paypal.com/docs/api/identity/v1/");
codegen::generate_model!("UserManagement", "https://developer.paypal.com/docs/api/identity/v2/");

codegen::generate_model!("Invoicing", "https://developer.paypal.com/docs/api/invoicing/v2/");
codegen::generate_model!("Orders", "https://developer.paypal.com/docs/api/orders/v2/");
codegen::generate_model!("PartnerReferrals", "https://developer.paypal.com/docs/api/partner-referrals/v2/");
codegen::generate_model!("PaymentExperience", "https://developer.paypal.com/docs/api/payment-experience/v1/");
codegen::generate_model!("PaymentTokens", "https://developer.paypal.com/docs/api/payment-tokens/v3/");
codegen::generate_model!("Payments", "https://developer.paypal.com/docs/api/payments/v2/");
codegen::generate_model!("PaymentsBatchPayouts", "https://developer.paypal.com/docs/api/payments.payouts-batch/v1/");
codegen::generate_model!("ReferencedPayouts", "https://developer.paypal.com/docs/api/referenced-payouts/v1/");
codegen::generate_model!("Subscriptions", "https://developer.paypal.com/docs/api/subscriptions/v1/");
codegen::generate_model!("TransactionSearch", "https://developer.paypal.com/docs/api/transaction-search/v1/");
codegen::generate_model!("Webhooks", "https://developer.paypal.com/docs/api/webhooks/v1/");