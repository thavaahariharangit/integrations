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

use anyhow::*;
use scraper::*;

use bitsnap_integrations_shared::codegen::*;
use bitsnap_integrations_shared::http::*;

use hyper_util::rt::TokioExecutor;

const SCRAPE_PAGE_HEADER_SELECTOR: &'static str = "div.page-header h2";
const SCRAPE_ROOT_FIELDS_GROUP_SELECTOR: &'static str = "div.cb-list-group";

pub(crate) async fn parse(url: String, executor: TokioExecutor) -> Result<API> {
    // let mut models = Vec::new();
    // let mut enums = Vec::new();

    // for url in urls {
    //     let document = Html::parse_document(&url);

    //     let header_selector = Selector::parse(Self::SCRAPE_PAGE_HEADER_SELECTOR).unwrap();
    //     let root_fields_selector =
    //         Selector::parse(Self::SCRAPE_ROOT_FIELDS_GROUP_SELECTOR).unwrap();

    //     let header_elem = document
    //         .select(&header_selector)
    //         .find(|h| h.text().collect::<String>().replace("¶", "").trim() == header)
    //         .expect(&format!("header {} not found", header));

    //     let root_model_node = header_elem
    //         .ancestors()
    //         .nth(2)
    //         .expect(&format!("root model {} not found", header));

    //     let root_fields_elem = ElementRef::wrap(root_model_node)
    //         .unwrap()
    //         .select(&root_fields_selector)
    //         .next()
    //         .expect(&format!("root model fields {} not found", header));
    // }

    Err(anyhow!("not implemented"))
}

// let document = Html::parse_document(&html);

// let header_selector = Selector::parse(Self::SCRAPE_PAGE_HEADER_SELECTOR).unwrap();
// let root_fields_selector =
//   Selector::parse(Self::SCRAPE_ROOT_FIELDS_GROUP_SELECTOR).unwrap();

// let header_elem = document
//   .select(&header_selector)
//   .find(|h| h.text().collect::<String>().replace("¶", "").trim() == header)
//   .expect(&format!("header {} not found", header));

// let root_model_node = header_elem
//   .ancestors()
//   .nth(2)
//   .expect(&format!("root model {} not found", header));

// let root_fields_elem = ElementRef::wrap(root_model_node)
//   .unwrap()
//   .select(&root_fields_selector)
//   .next()
//   .expect(&format!("root model fields {} not found", header));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_names() {
        let result = model_names(vec![String::from(
            "https://chargebee.com/docs/api/2.0/subscription.html",
        )]);
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }
}
