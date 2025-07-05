/*
    Copyright (C) 2017-2025 Yuriy Yarosh.
    All rights reserved.
*/

use bitsnap_integration::Client as HttpsClient;

pub struct ChargeBee {
    pub(crate) base_url: String,
    pub(crate) client: HttpsClient,
    pub(crate) token: String,
    pub(crate) site: String,
}

const DEFAULT_BASE_URL: &str = "chargebee.com/api/v2";

impl ChargeBee {
    fn new(token: String, site: String) -> Self {
        ChargeBee {
            base_url: String::from(DEFAULT_BASE_URL),
            client: HttpsClient::new(),
            token,
            site,
        }
    }

    // // https://apidocs.chargebee.com/docs/api/business_entities?lang=curl#list_the_business_entity_transfers
    // codegen::generate_list!("business_entity_transfers", "BusinessEntityTransfer", "/business_entities/transfers");
    //
    // // https://apidocs.chargebee.com/docs/api/currencies?lang=curl#list_currencies
    // codegen::generate_list!("contacts", "Contact", "/contacts");
    //
    // // https://apidocs.chargebee.com/docs/api/currencies
    // codegen::generate_list!("currencies", "Currency", "/currencies");
    // codegen::generate_retrieve!("Currency", "/currencies");
    //
    // // https://apidocs.chargebee.com/docs/api/customers
    // codegen::generate_list!("customers", "Customer", "/customers");
    // codegen::generate_retrieve!("Customer", "/customers");
    //
    // // https://apidocs.chargebee.com/docs/api/customers?lang=curl#list_of_contacts_for_a_customer
    // codegen::generate_list_of!("customer_contacts", "Contact", "/customers", "contacts");
    //
    // // https://apidocs.chargebee.com/docs/api/hierarchies
    // codegen::generate_list_of!("customer_hierarchies", "Hierarchy", "/customers", "hierarchy");
    //
    // // https://apidocs.chargebee.com/docs/api/payment_vouchers?lang=curl#list_vouchers_for_an_invoice
    // codegen::generate_list_of!("customer_payment_vouchers", "PaymentVoucher", "/customers", "payment_vouchers");
    //
    // // https://apidocs.chargebee.com/docs/api/entitlements
    // codegen::generate_list!("entitlements", "Entitlement", "/entitlements");
    //
    // // https://apidocs.chargebee.com/docs/api/events
    // codegen::generate_list!("events", "Event", "/events");
    // codegen::generate_retrieve!("Event", "/events");
    //
    // // https://apidocs.chargebee.com/docs/api/features
    // codegen::generate_list!("features", "Feature", "/features");
    // codegen::generate_retrieve!("Feature", "/features");
    //
    // // https://apidocs.chargebee.com/docs/api/invoices
    // codegen::generate_list!("invoices", "Invoice", "/invoices");
    // codegen::generate_retrieve!("Invoice", "/invoices");
    //
    // // https://apidocs.chargebee.com/docs/api/credit_notes
    // codegen::generate_list!("credit_notes", "CreditNote", "/credit_notes");
    // codegen::generate_retrieve!("CreditNote", "/credit_notes");
    //
    // // https://apidocs.chargebee.com/docs/api/payment_reference_numbers
    // // https://apidocs.chargebee.com/docs/api/invoices?lang=curl#list_payment_reference_numbers
    // // GenerateListChildrenWithId("PaymentReferenceNumbers", "PaymentReferenceNumber", "/invoices", "payment_reference_numbers", false, "")
    //
    // // https://apidocs.chargebee.com/docs/api/transactions?lang=curl#list_payments_for_an_invoice
    // codegen::generate_list_of!("invoice_transactions", "Transaction", "/invoices", "payments");
    //
    // // https://apidocs.chargebee.com/docs/api/promotional_credits
    // codegen::generate_retrieve!("PromotionalCredit", "/promotional_credits");
    //
    // // https://apidocs.chargebee.com/docs/api/unbilled_charges
    // codegen::generate_list!("invoice_un_billed_Charges", "InvoiceUnBilledCharge", "/unbilled_charges");
    //
    // // https://apidocs.chargebee.com/docs/api/payment_vouchers?lang=curl#list_vouchers_for_an_invoice
    // codegen::generate_list_of!("invoice_payment_vouchers", "PaymentVoucher", "/invoices", "payment_vouchers");
    //
    // // https://apidocs.chargebee.com/docs/api/items
    // codegen::generate_list!("Items", "Item", "/items");
    // codegen::generate_retrieve!("Item", "/items");
    //
    // // https://apidocs.chargebee.com/docs/api/coupon_codes
    // codegen::generate_list!("CouponCodes", "CouponCode", "/coupon_codes");
    // codegen::generate_retrieve!("CouponCode", "/coupon_codes");
    //
    // // https://apidocs.chargebee.com/docs/api/coupon_sets
    // codegen::generate_list!("CouponSets", "CouponSet", "/coupon_sets");
    // codegen::generate_retrieve!("CouponSet", "/coupon_sets");
    //
    // // https://apidocs.chargebee.com/docs/api/coupons
    // codegen::generate_list!("Coupons", "Coupon", "/coupons");
    // codegen::generate_retrieve!("Coupon", "/coupons");
    //
    // // https://apidocs.chargebee.com/docs/api/differential_prices
    // codegen::generate_list!("DifferentialPrices", "DifferentialPrice", "/item_prices");
    // codegen::generate_retrieve!("DifferentialPrice", "/item_prices");
    //
    // // https://apidocs.chargebee.com/docs/api/item_families
    // codegen::generate_list!("ItemFamilies", "ItemFamily", "/item_families");
    // codegen::generate_retrieve!("ItemFamily", "/item_families");
    //
    // // https://apidocs.chargebee.com/docs/api/item_prices
    // codegen::generate_list!("ItemPrices", "ItemPrice", "/item_prices");
    // codegen::generate_retrieve!("ItemPrice", "/item_prices");
    //
    // // https://apidocs.chargebee.com/docs/api/attached_items
    // codegen::generate_list_of!("items_attached", "ItemAttached", "/items", "attached_items");
    // codegen::generate_retrieve!("ItemAttached", "/attached_items");
    //
    // // https://apidocs.chargebee.com/docs/api/orders?lang=curl#list_orders
    // codegen::generate_list!("orders", "Order", "/orders");
    // codegen::generate_retrieve!("Order", "/orders");
    //
    // // https://apidocs.chargebee.com/docs/api/transactions?lang=curl#list_transactions
    // codegen::generate_list!("transactions", "Transaction", "/transactions");
    // codegen::generate_retrieve!("Transaction", "/transactions");
    //
    // // https://apidocs.chargebee.com/docs/api/payment_sources?lang=curl#list_payment_sources
    // codegen::generate_list!("PaymentSources", "PaymentSource", "/payment_sources");
    // codegen::generate_retrieve!("PaymentSource", "/payment_sources");
    //
    // // https://apidocs.chargebee.com/docs/api/virtual_bank_accounts
    // codegen::generate_list!("virtual_bank_accounts", "VirtualBankAccount", "/virtual_bank_accounts");
    // codegen::generate_retrieve!("VirtualBankAccount", "/virtual_bank_accounts");
    //
    // // https://apidocs.chargebee.com/docs/api/quotes?lang=curl#list_quotes
    // codegen::generate_list!("quotes", "Quote", "/quotes");
    // codegen::generate_retrieve!("Quote", "/quotes");
    //
    // // https://apidocs.chargebee.com/docs/api/quotes?lang=curl#list_quote_line_groups
    // codegen::generate_list_of!("quote_line_groups", "QuoteLineGroup", "/quotes", "quote_line_groups");
    // codegen::generate_list!("all_quote_line_groups", "QuoteLineGroup", "/quote_line_groups");
    //
    // // https://apidocs.chargebee.com/docs/api/subscriptions?lang=curl#list_subscriptions
    // codegen::generate_list!("subscriptions", "Subscription", "/subscriptions");
    // codegen::generate_list_of!("subscription_with_scheduled_changes", "Subscription", "/subscriptions", "retrieve_with_scheduled_changes");
    //
    // // https://apidocs.chargebee.com/docs/api/addresses?lang=curl#retrieve_an_address
    // //GenerateListChildrenWithId("SubscriptionAddresses", "Address", "/addresses", "", false, "subscription_id")
    //
    // // https://apidocs.chargebee.com/docs/api/subscriptions?lang=curl#list_contract_terms_for_a_subscription
    // codegen::generate_list_of!("contract_terms", "ContractTerm", "/subscriptions", "contract_terms");
    //
    // // https://apidocs.chargebee.com/docs/api/subscriptions?lang=curl#list_discounts_for_a_subscription
    // codegen::generate_list_of!("discounts", "Discount", "/subscriptions", "discounts");
    //
    // // https://apidocs.chargebee.com/docs/api/subscription_entitlements?lang=curl#list_subscription_entitlements
    // codegen::generate_list_of!("subscription_entitlements", "SubscriptionEntitlement", "/subscriptions", "subscription_entitlements");
    //
    // // https://apidocs.chargebee.com/docs/api/gifts?lang=curl#retrieve_a_gift
    // codegen::generate_list!("gifts", "Gift", "/gifts");
    // codegen::generate_retrieve!("Gift", "/gifts");
    //
    // // https://apidocs.chargebee.com/docs/api/subscriptions?lang=curl#retrieve_advance_invoice
    // codegen::generate_list_of!("advanced_invoice_schedules", "AdvancedInvoiceSchedule", "/subscriptions", "retrieve_advance_invoice_schedule");
    //
    // // https://apidocs.chargebee.com/docs/api/ramps?lang=curl#list_ramps
    // codegen::generate_list!("ramps", "Ramp", "/ramps");
    //
    // // https://apidocs.chargebee.com/docs/api/usages?lang=curl#list_usages
    // codegen::generate_list!("usages", "Usage", "/usages");
    //
    // // https://apidocs.chargebee.com/docs/api/usages?lang=curl#retrieve_a_usage
    // codegen::generate_list_of!("subscription_usages", "Usage", "/subscription", "usages");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn client_should_get() {}
}
