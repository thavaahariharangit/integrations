/*
 * Copyright (C) 2016-2025 Yuriy Yarosh
 * All rights reserved.
 *
 * SPDX-License-Identifier: MPL-2.0
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

mod parser;

use anyhow::*;

use bitsnap_integrations_shared::codegen::*;

use futures::stream;
use futures::StreamExt;
use hyper_util::rt::TokioExecutor;
use itertools::Itertools;

pub async fn save(dir: &str) -> Result<Vec<API>> {

    let executor = TokioExecutor::new();
    let apis = stream::iter(vec![
          "https://apidocs.chargebee.com/docs/api/advance_invoice_schedules",
          "https://apidocs.chargebee.com/docs/api/attached_items",
          "https://apidocs.chargebee.com/docs/api/business_entities",
          "https://apidocs.chargebee.com/docs/api/business_entity_transfers",
          "https://apidocs.chargebee.com/docs/api/contacts",
          "https://apidocs.chargebee.com/docs/api/contract_terms",
          "https://apidocs.chargebee.com/docs/api/coupon_codes",
          "https://apidocs.chargebee.com/docs/api/coupons",
          "https://apidocs.chargebee.com/docs/api/coupon_sets",
          "https://apidocs.chargebee.com/docs/api/credit_notes",
          "https://apidocs.chargebee.com/docs/api/currencies",
          "https://apidocs.chargebee.com/docs/api/customers",
          "https://apidocs.chargebee.com/docs/api/differential_prices",
          "https://apidocs.chargebee.com/docs/api/discounts",
          "https://apidocs.chargebee.com/docs/api/entitlements",
          "https://apidocs.chargebee.com/docs/api/events",
          "https://apidocs.chargebee.com/docs/api/features",
          "https://apidocs.chargebee.com/docs/api/gifts",
          "https://apidocs.chargebee.com/docs/api/hierarchies",
          "https://apidocs.chargebee.com/docs/api/impacted_items",
          "https://apidocs.chargebee.com/docs/api/impacted_subscriptions",
          "https://apidocs.chargebee.com/docs/api/invoices",
          "https://apidocs.chargebee.com/docs/api/item_families",
          "https://apidocs.chargebee.com/docs/api/item_prices",
          "https://apidocs.chargebee.com/docs/api/items",
          "https://apidocs.chargebee.com/docs/api/orders",
          "https://apidocs.chargebee.com/docs/api/payment_reference_numbers",
          "https://apidocs.chargebee.com/docs/api/payment_sources",
          "https://apidocs.chargebee.com/docs/api/payment_vouchers",
          "https://apidocs.chargebee.com/docs/api/promotional_credits",
          "https://apidocs.chargebee.com/docs/api/quoted_charges",
          "https://apidocs.chargebee.com/docs/api/quoted_subscriptions",
          "https://apidocs.chargebee.com/docs/api/quote_line_groups",
          "https://apidocs.chargebee.com/docs/api/quotes",
          "https://apidocs.chargebee.com/docs/api/ramps",
          "https://apidocs.chargebee.com/docs/api/subscription_entitlements",
          "https://apidocs.chargebee.com/docs/api/subscriptions",
          "https://apidocs.chargebee.com/docs/api/transactions",
          "https://apidocs.chargebee.com/docs/api/unbilled_charges",
          "https://apidocs.chargebee.com/docs/api/usages",
          "https://apidocs.chargebee.com/docs/api/virtual_bank_accounts",
      ])
      .map(|url| {
        let executor = executor.clone();
        async move { parser::parse(url.to_string(), executor).await }
      })
      .buffer_unordered(8)
      .collect::<Vec<Result<API>>>()
      .await;

    let (result, errors): (Vec<_>, Vec<_>) = apis.into_iter().partition_result();

    if errors.is_empty() {
        Ok(result)
    } else {
        for error in &errors {
            println!("cargo:warning=Failed to parse API doc: {error}");
        }
        Err(anyhow::anyhow!(
            "failed to parse {} API docs",
            errors.len()
        ))
    }
}

pub fn load(dir: &str) -> Result<Vec<API>> {
    Err(anyhow!("not implemented"))
}
