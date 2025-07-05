/*
    Copyright (C) 2017-2025 Yuriy Yarosh.
    All rights reserved.
*/

use bitsnap_codegen_util::*;
use bitsnap_integration::Client as HttpsClient;

use proc_macro2::{Ident, Span, TokenStream};
use quote::*;
use scraper::*;
use std::fs;
use std::path::PathBuf;
use syn::{LitStr, Token, parse, parse_macro_input};

pub(crate) mod codegen;
pub(crate) mod fields;
use fields::*;
pub(crate) mod validation;

#[derive(Default, Debug, Clone)]
pub(crate) struct Model {
    pub name: String,
    pub fields: Vec<Field>,
}

impl Model {
    const SCRAPE_PAGE_HEADER_SELECTOR: &'static str = "div.page-header h2";
    const SCRAPE_ROOT_FIELDS_GROUP_SELECTOR: &'static str = "div.cb-list-group";

    fn scrape(html: String, name: &str, header: &str) -> codegen::Result<Model> {
        let document = Html::parse_document(&html);

        let header_selector = Selector::parse(Self::SCRAPE_PAGE_HEADER_SELECTOR).unwrap();
        let root_fields_selector =
            Selector::parse(Self::SCRAPE_ROOT_FIELDS_GROUP_SELECTOR).unwrap();

        let header_elem = document
            .select(&header_selector)
            .find(|h| h.text().collect::<String>().replace("¶", "").trim() == header)
            .expect(&format!("header {} not found", header));

        let root_model_node = header_elem
            .ancestors()
            .nth(2)
            .expect(&format!("root model {} not found", header));

        let root_fields_elem = ElementRef::wrap(root_model_node)
            .unwrap()
            .select(&root_fields_selector)
            .next()
            .expect(&format!("root model fields {} not found", header));

        // let name = snake_case_to_camel_case(&*header.replace("attributes", "").trim().to_string().replace(" ", "_"));

        Ok(Model {
            name: name.to_string(),
            fields: Field::new(&root_fields_elem, &name),
        })
    }

    fn generate(&self) -> TokenStream {
        let _: Vec<_> = self
            .fields
            .iter()
            .map(|field| {
                field.clone().generate_enums_file();
                field.clone().generate_submodel_file();
            })
            .collect();

        let name_ident = Ident::new(&self.name, Span::call_site());

        let fields: Vec<_> = self
            .fields
            .iter()
            .map(|field| field.generate(&self.name))
            .collect();

        let default_value_fn: Vec<_> = self
            .fields
            .iter()
            .map(|field| field.generate_default_func())
            .collect();

        let mut modules: Vec<_> = self
            .fields
            .iter()
            .filter_map(|f| {
                if f.is_model || f.is_enum {
                    f.submodel.clone().map(|s| (s, f.is_enum))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        modules.dedup_by(|a, b| a == b);

        let modules: Vec<_> = modules
            .iter()
            .map(|(submodel, is_enum)| {
                let module_name =
                    Ident::new(&(camel_case_to_snake_case(&submodel)), Span::call_site());

                if *is_enum {
                    quote! {
                        use crate::enums::#module_name::*;
                    }
                } else {
                    quote! {
                        mod #module_name;
                        use #module_name::*;
                    }
                }
            })
            .collect();

        quote! {
            #(#modules)*

            #[derive(Default, Debug, Clone, Serialize, Deserialize, Validate)]
            #[serde(rename_all = "snake_case")]
            pub struct #name_ident {
                #(#fields),*
            }

            impl #name_ident {
                #(#default_value_fn)*
            }
        }
    }
}

/// Macro Input
#[derive(Debug)]
struct Input {
    name: LitStr,
    header: LitStr,
    url: LitStr,
}

impl parse::Parse for Input {
    fn parse(input: parse::ParseStream) -> syn::Result<Input> {
        let name: LitStr = input.parse()?;
        input.parse::<Token![,]>()?;
        let header: LitStr = input.parse()?;
        input.parse::<Token![,]>()?;
        let url: LitStr = input.parse()?;
        Ok(Input { name, header, url })
    }
}

pub(crate) fn generate(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let Input { name, header, url } = parse_macro_input!(input as Input);
    let model = scrape_models(&name.value(), &header.value(), &url.value());

    let expanded = model.unwrap().generate();

    // eprintln!("{}", expanded);

    expanded.into()
}

pub(crate) fn scrape_models(name: &str, header: &str, url: &str) -> codegen::Result<Model> {
    let html = fs::read_to_string(HttpsClient::scrape_file_path(&url, "chargebee"))
        .expect("Failed to read HTML file");
    Model::scrape(html, name, &header)
}

pub fn generate_enums_rs() -> proc_macro::TokenStream {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let enums_dir = dir.join("../../clients/accounting_chargebee/src/enums");
    let enums_rs = dir.join("../../clients/accounting_chargebee/src/enums.rs");

    if enums_dir.exists() && !enums_rs.exists() {
        let contents = enums_dir.read_dir().expect("Failed to read directory");
        let decls = contents
            .into_iter()
            .filter_map(|e| e.ok())
            .map(|f| {
                let f = f
                    .file_name()
                    .clone()
                    .to_str()
                    .unwrap()
                    .to_owned()
                    .replace(".rs", "");
                let mod_ = format!("pub mod {};", f);

                mod_
            })
            .collect::<Vec<_>>()
            .join("\n");

        let _ = fs::write(
            enums_rs,
            "/* DO NOT EDIT. THIS FILE IS AUTOMATICALLY GENERATED */\n\n".to_string() + &decls,
        );
    }

    quote!().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn business_entity() {
        goldie::assert!(
            scrape_models(
                "BusinessEntity",
                "Business entity attributes",
                "https://apidocs.chargebee.com/docs/api/business_entities"
            )
            .unwrap()
            .generate()
            .to_string()
        );
    }

    #[test]
    fn business_entity_transfer() {
        goldie::assert!(
            scrape_models(
                "BusinessEntityTransfer",
                "Business entity transfer attributes",
                "https://apidocs.chargebee.com/docs/api/business_entity_transfers"
            )
            .unwrap()
            .generate()
            .to_string()
        );
    }

    #[test]
    fn contact() {
        goldie::assert!(
            scrape_models(
                "Contact",
                "Contact attributes",
                "https://apidocs.chargebee.com/docs/api/contacts"
            )
            .unwrap()
            .generate()
            .to_string()
        );
    }

    #[test]
    fn currency() {
        goldie::assert!(
            scrape_models(
                "Currency",
                "Currency attributes",
                "https://apidocs.chargebee.com/docs/api/currencies"
            )
            .unwrap()
            .generate()
            .to_string()
        );
    }

    #[test]
    fn customer() {
        goldie::assert!(
            scrape_models(
                "Customer",
                "Customer attributes",
                "https://apidocs.chargebee.com/docs/api/customers"
            )
            .unwrap()
            .generate()
            .to_string()
        );
    }

    #[test]
    fn hierarchy() {
        goldie::assert!(
            scrape_models(
                "Hierarchy",
                "Hierarchy attributes",
                "https://apidocs.chargebee.com/docs/api/hierarchies"
            )
            .unwrap()
            .generate()
            .to_string()
        );
    }

    #[test]
    fn entitlement() {
        goldie::assert!(
            scrape_models(
                "Entitlement",
                "Entitlement attributes",
                "https://apidocs.chargebee.com/docs/api/entitlements"
            )
            .unwrap()
            .generate()
            .to_string()
        );
    }

    #[test]
    fn event() {
        goldie::assert!(
            scrape_models(
                "Event",
                "Event attributes",
                "https://apidocs.chargebee.com/docs/api/events"
            )
            .unwrap()
            .generate()
            .to_string()
        );
    }

    #[test]
    fn impacted_item() {
        goldie::assert!(
            scrape_models(
                "ImpactedItem",
                "Impacted item attributes",
                "https://apidocs.chargebee.com/docs/api/impacted_items"
            )
            .unwrap()
            .generate()
            .to_string()
        );
    }

    #[test]
    fn impacted_subscription() {
        goldie::assert!(
            scrape_models(
                "ImpactedSubscription",
                "Impacted subscription attributes",
                "https://apidocs.chargebee.com/docs/api/impacted_subscriptions"
            )
            .unwrap()
            .generate()
            .to_string()
        );
    }

    #[test]
    fn feature() {
        goldie::assert!(
            scrape_models(
                "Feature",
                "Feature attributes",
                "https://apidocs.chargebee.com/docs/api/features"
            )
            .unwrap()
            .generate()
            .to_string()
        );
    }

    #[test]
    fn invoice() {
        goldie::assert!(
            scrape_models(
                "Invoice",
                "Invoice attributes",
                "https://apidocs.chargebee.com/docs/api/invoices"
            )
            .unwrap()
            .generate()
            .to_string()
        );
    }

    #[test]
    fn credit_note() {
        goldie::assert!(
            scrape_models(
                "CreditNote",
                "Credit note attributes",
                "https://apidocs.chargebee.com/docs/api/credit_notes"
            )
            .unwrap()
            .generate()
            .to_string()
        );
    }

    #[test]
    fn payment_reference_number() {
        goldie::assert!(
            scrape_models(
                "PaymentReferenceNumber",
                "Payment reference number attributes",
                "https://apidocs.chargebee.com/docs/api/payment_reference_numbers"
            )
            .unwrap()
            .generate()
            .to_string()
        );
    }

    #[test]
    fn promotional_credit() {
        goldie::assert!(
            scrape_models(
                "PromotionalCredit",
                "Promotional credit attributes",
                "https://apidocs.chargebee.com/docs/api/promotional_credits"
            )
            .unwrap()
            .generate()
            .to_string()
        );
    }

    #[test]
    fn invoice_un_billed_charge() {
        goldie::assert!(
            scrape_models(
                "InvoiceUnBilledCharge",
                "Unbilled charge attributes",
                "https://apidocs.chargebee.com/docs/api/unbilled_charges"
            )
            .unwrap()
            .generate()
            .to_string()
        );
    }

    #[test]
    fn item() {
        goldie::assert!(
            scrape_models(
                "Item",
                "Item attributes",
                "https://apidocs.chargebee.com/docs/api/items"
            )
            .unwrap()
            .generate()
            .to_string()
        );
    }

    #[test]
    fn coupon_code() {
        goldie::assert!(
            scrape_models(
                "CouponCode",
                "Coupon code attributes",
                "https://apidocs.chargebee.com/docs/api/coupon_codes"
            )
            .unwrap()
            .generate()
            .to_string()
        );
    }

    #[test]
    fn coupon_set() {
        goldie::assert!(
            scrape_models(
                "CouponSet",
                "Coupon set attributes",
                "https://apidocs.chargebee.com/docs/api/coupon_sets"
            )
            .unwrap()
            .generate()
            .to_string()
        );
    }

    #[test]
    fn coupon() {
        goldie::assert!(
            scrape_models(
                "Coupon",
                "Coupon attributes",
                "https://apidocs.chargebee.com/docs/api/coupons"
            )
            .unwrap()
            .generate()
            .to_string()
        );
    }

    #[test]
    fn differential_price() {
        goldie::assert!(
            scrape_models(
                "DifferentialPrice",
                "Differential price attributes",
                "https://apidocs.chargebee.com/docs/api/differential_prices"
            )
            .unwrap()
            .generate()
            .to_string()
        );
    }

    #[test]
    fn item_family() {
        goldie::assert!(
            scrape_models(
                "ItemFamily",
                "Item family attributes",
                "https://apidocs.chargebee.com/docs/api/item_families"
            )
            .unwrap()
            .generate()
            .to_string()
        );
    }

    #[test]
    fn item_price() {
        goldie::assert!(
            scrape_models(
                "ItemPrice",
                "Item price attributes",
                "https://apidocs.chargebee.com/docs/api/item_prices"
            )
            .unwrap()
            .generate()
            .to_string()
        );
    }

    #[test]
    fn item_attached() {
        goldie::assert!(
            scrape_models(
                "ItemAttached",
                "Attached item attributes",
                "https://apidocs.chargebee.com/docs/api/attached_items"
            )
            .unwrap()
            .generate()
            .to_string()
        );
    }

    #[test]
    fn order() {
        goldie::assert!(
            scrape_models(
                "Order",
                "Order attributes",
                "https://apidocs.chargebee.com/docs/api/orders"
            )
            .unwrap()
            .generate()
            .to_string()
        );
    }

    #[test]
    fn transaction() {
        goldie::assert!(
            scrape_models(
                "Transaction",
                "Transaction attributes",
                "https://apidocs.chargebee.com/docs/api/transactions"
            )
            .unwrap()
            .generate()
            .to_string()
        );
    }

    #[test]
    fn payment_source() {
        goldie::assert!(
            scrape_models(
                "PaymentSource",
                "Payment source attributes",
                "https://apidocs.chargebee.com/docs/api/payment_sources"
            )
            .unwrap()
            .generate()
            .to_string()
        );
    }

    #[test]
    fn virtual_bank_account() {
        goldie::assert!(
            scrape_models(
                "VirtualBankAccount",
                "Virtual bank account attributes",
                "https://apidocs.chargebee.com/docs/api/virtual_bank_accounts"
            )
            .unwrap()
            .generate()
            .to_string()
        );
    }

    #[test]
    fn payment_voucher() {
        goldie::assert!(
            scrape_models(
                "PaymentVoucher",
                "Payment voucher attributes",
                "https://apidocs.chargebee.com/docs/api/payment_vouchers"
            )
            .unwrap()
            .generate()
            .to_string()
        );
    }

    #[test]
    fn quote() {
        goldie::assert!(
            scrape_models(
                "Quote",
                "Quote attributes",
                "https://apidocs.chargebee.com/docs/api/quotes"
            )
            .unwrap()
            .generate()
            .to_string()
        );
    }

    #[test]
    fn quote_line_group() {
        goldie::assert!(
            scrape_models(
                "QuoteLineGroup",
                "Quote line group attributes",
                "https://apidocs.chargebee.com/docs/api/quote_line_groups"
            )
            .unwrap()
            .generate()
            .to_string()
        );
    }

    #[test]
    fn quoted_charge() {
        goldie::assert!(
            scrape_models(
                "QuotedCharge",
                "Quoted charge attributes",
                "https://apidocs.chargebee.com/docs/api/quoted_charges"
            )
            .unwrap()
            .generate()
            .to_string()
        );
    }

    #[test]
    fn quoted_subscription() {
        goldie::assert!(
            scrape_models(
                "QuotedSubscription",
                "Quoted subscription attributes",
                "https://apidocs.chargebee.com/docs/api/quoted_subscriptions"
            )
            .unwrap()
            .generate()
            .to_string()
        );
    }

    #[test]
    fn subscription() {
        goldie::assert!(
            scrape_models(
                "Subscription",
                "Subscription attributes",
                "https://apidocs.chargebee.com/docs/api/subscriptions"
            )
            .unwrap()
            .generate()
            .to_string()
        );
    }

    #[test]
    fn contract_term() {
        goldie::assert!(
            scrape_models(
                "ContractTerm",
                "Contract term attributes",
                "https://apidocs.chargebee.com/docs/api/contract_terms"
            )
            .unwrap()
            .generate()
            .to_string()
        );
    }

    #[test]
    fn discount() {
        goldie::assert!(
            scrape_models(
                "Discount",
                "Discount attributes",
                "https://apidocs.chargebee.com/docs/api/discounts"
            )
            .unwrap()
            .generate()
            .to_string()
        );
    }

    #[test]
    fn subscription_entitlement() {
        goldie::assert!(
            scrape_models(
                "SubscriptionEntitlement",
                "Subscription entitlement attributes",
                "https://apidocs.chargebee.com/docs/api/subscription_entitlements"
            )
            .unwrap()
            .generate()
            .to_string()
        );
    }

    #[test]
    fn gift() {
        goldie::assert!(
            scrape_models(
                "Gift",
                "Gift attributes",
                "https://apidocs.chargebee.com/docs/api/gifts"
            )
            .unwrap()
            .generate()
            .to_string()
        );
    }

    #[test]
    fn advanced_invoice_schedule() {
        goldie::assert!(
            scrape_models(
                "AdvancedInvoiceSchedule",
                "Advance invoice schedule attributes",
                "https://apidocs.chargebee.com/docs/api/advance_invoice_schedules"
            )
            .unwrap()
            .generate()
            .to_string()
        );
    }

    #[test]
    fn ramp() {
        goldie::assert!(
            scrape_models(
                "Ramp",
                "Ramp attributes",
                "https://apidocs.chargebee.com/docs/api/ramps"
            )
            .unwrap()
            .generate()
            .to_string()
        );
    }

    #[test]
    fn usage() {
        goldie::assert!(
            scrape_models(
                "Usage",
                "Usage attributes",
                "https://apidocs.chargebee.com/docs/api/usages"
            )
            .unwrap()
            .generate()
            .to_string()
        );
    }
}
