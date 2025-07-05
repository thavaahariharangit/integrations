/*
    Copyright (C) 2017-2025 Yuriy Yarosh.
    All rights reserved.
*/

use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use serde_valid::Validate;

use bitsnap_accounting_chargebee_macro as codegen;

codegen::generate_model!(
    "BusinessEntity",
    "Business entity attributes",
    "https://apidocs.chargebee.com/docs/api/business_entities"
);

codegen::generate_model!(
    "BusinessEntityTransfer",
    "Business entity transfer attributes",
    "https://apidocs.chargebee.com/docs/api/business_entity_transfers"
);

codegen::generate_model!(
    "Contact",
    "Contact attributes",
    "https://apidocs.chargebee.com/docs/api/contacts"
);

codegen::generate_model!(
    "Currency",
    "Currency attributes",
    "https://apidocs.chargebee.com/docs/api/currencies"
);

// Customer
codegen::generate_model!(
    "Customer",
    "Customer attributes",
    "https://apidocs.chargebee.com/docs/api/customers"
);
codegen::generate_model!(
    "Hierarchy",
    "Hierarchy attributes",
    "https://apidocs.chargebee.com/docs/api/hierarchies"
);
codegen::generate_model!(
    "Entitlement",
    "Entitlement attributes",
    "https://apidocs.chargebee.com/docs/api/entitlements"
);

// Events
codegen::generate_model!(
    "Event",
    "Event attributes",
    "https://apidocs.chargebee.com/docs/api/events"
);
codegen::generate_model!(
    "ImpactedItem",
    "Impacted item attributes",
    "https://apidocs.chargebee.com/docs/api/impacted_items"
);
codegen::generate_model!(
    "ImpactedSubscription",
    "Impacted subscription attributes",
    "https://apidocs.chargebee.com/docs/api/impacted_subscriptions"
);

codegen::generate_model!(
    "Feature",
    "Feature attributes",
    "https://apidocs.chargebee.com/docs/api/features"
);

// Invoice
codegen::generate_model!(
    "Invoice",
    "Invoice attributes",
    "https://apidocs.chargebee.com/docs/api/invoices"
);
codegen::generate_model!(
    "CreditNote",
    "Credit note attributes",
    "https://apidocs.chargebee.com/docs/api/credit_notes"
);
codegen::generate_model!(
    "PaymentReferenceNumber",
    "Payment reference number attributes",
    "https://apidocs.chargebee.com/docs/api/payment_reference_numbers"
);
codegen::generate_model!(
    "PromotionalCredit",
    "Promotional credit attributes",
    "https://apidocs.chargebee.com/docs/api/promotional_credits"
);
codegen::generate_model!(
    "InvoiceUnBilledCharge",
    "Unbilled charge attributes",
    "https://apidocs.chargebee.com/docs/api/unbilled_charges"
);

// Items
codegen::generate_model!(
    "Item",
    "Item attributes",
    "https://apidocs.chargebee.com/docs/api/items"
);
codegen::generate_model!(
    "CouponCode",
    "Coupon code attributes",
    "https://apidocs.chargebee.com/docs/api/coupon_codes"
);
codegen::generate_model!(
    "CouponSet",
    "Coupon set attributes",
    "https://apidocs.chargebee.com/docs/api/coupon_sets"
);
codegen::generate_model!(
    "Coupon",
    "Coupon attributes",
    "https://apidocs.chargebee.com/docs/api/coupons"
);
codegen::generate_model!(
    "DifferentialPrice",
    "Differential price attributes",
    "https://apidocs.chargebee.com/docs/api/differential_prices"
);
codegen::generate_model!(
    "ItemFamily",
    "Item family attributes",
    "https://apidocs.chargebee.com/docs/api/item_families"
);
codegen::generate_model!(
    "ItemPrice",
    "Item price attributes",
    "https://apidocs.chargebee.com/docs/api/item_prices"
);
codegen::generate_model!(
    "ItemAttached",
    "Attached item attributes",
    "https://apidocs.chargebee.com/docs/api/attached_items"
);

codegen::generate_model!(
    "Order",
    "Order attributes",
    "https://apidocs.chargebee.com/docs/api/orders"
);

// Payments
codegen::generate_model!(
    "Transaction",
    "Transaction attributes",
    "https://apidocs.chargebee.com/docs/api/transactions"
);
codegen::generate_model!(
    "PaymentSource",
    "Payment source attributes",
    "https://apidocs.chargebee.com/docs/api/payment_sources"
);
codegen::generate_model!(
    "VirtualBankAccount",
    "Virtual bank account attributes",
    "https://apidocs.chargebee.com/docs/api/virtual_bank_accounts"
);
codegen::generate_model!(
    "PaymentVoucher",
    "Payment voucher attributes",
    "https://apidocs.chargebee.com/docs/api/payment_vouchers"
);

// Quote
codegen::generate_model!(
    "Quote",
    "Quote attributes",
    "https://apidocs.chargebee.com/docs/api/quotes"
);
codegen::generate_model!(
    "QuoteLineGroup",
    "Quote line group attributes",
    "https://apidocs.chargebee.com/docs/api/quote_line_groups"
);
codegen::generate_model!(
    "QuotedCharge",
    "Quoted charge attributes",
    "https://apidocs.chargebee.com/docs/api/quoted_charges"
);
codegen::generate_model!(
    "QuotedSubscription",
    "Quoted subscription attributes",
    "https://apidocs.chargebee.com/docs/api/quoted_subscriptions"
);

// Subscription
codegen::generate_model!(
    "Subscription",
    "Subscription attributes",
    "https://apidocs.chargebee.com/docs/api/subscriptions"
);
codegen::generate_model!(
    "ContractTerm",
    "Contract term attributes",
    "https://apidocs.chargebee.com/docs/api/contract_terms"
);
codegen::generate_model!(
    "Discount",
    "Discount attributes",
    "https://apidocs.chargebee.com/docs/api/discounts"
);
codegen::generate_model!(
    "SubscriptionEntitlement",
    "Subscription entitlement attributes",
    "https://apidocs.chargebee.com/docs/api/subscription_entitlements"
);
codegen::generate_model!(
    "Gift",
    "Gift attributes",
    "https://apidocs.chargebee.com/docs/api/gifts"
);
codegen::generate_model!(
    "AdvancedInvoiceSchedule",
    "Advance invoice schedule attributes",
    "https://apidocs.chargebee.com/docs/api/advance_invoice_schedules"
);
codegen::generate_model!(
    "Ramp",
    "Ramp attributes",
    "https://apidocs.chargebee.com/docs/api/ramps"
);
codegen::generate_model!(
    "Usage",
    "Usage attributes",
    "https://apidocs.chargebee.com/docs/api/usages"
);

// NOTE: may break incremental builds
codegen::generate_enums_rs!();
