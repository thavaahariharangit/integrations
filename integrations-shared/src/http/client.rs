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

use anyhow::{Context, Result};
use http_body_util::Full;
use hyper::body::{Bytes, Incoming as IncomingBody};
use hyper::{Method, Request, Response, Uri};
use hyper_rustls::HttpsConnectorBuilder;
use hyper_util::{client::legacy::Client, rt::TokioExecutor};
use once_cell::sync::Lazy;

/// HTTP client for making requests with hyper.
#[derive(Clone, Debug)]
pub struct HttpClient {
    client: Client<
        hyper_rustls::HttpsConnector<hyper_util::client::legacy::connect::HttpConnector>,
        Full<Bytes>,
    >,
}

static INIT_RUSTLS_PROVIDER: Lazy<()> = Lazy::new(|| {
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
});

impl HttpClient {
    /// Creates a new `HttpClient` with an optional custom executor.
    pub fn new_with_executor(executor: Option<TokioExecutor>) -> Self {
        Lazy::force(&INIT_RUSTLS_PROVIDER);

        let https = HttpsConnectorBuilder::new()
            .with_native_roots()
            .expect("Failed to load native root CA certificates")
            .https_or_http()
            .enable_http1()
            .enable_http2()
            .build();

        Self {
            client: Client::builder(executor.unwrap_or_else(TokioExecutor::new)).build(https),
        }
    }

    /// Creates a new `HttpClient` with the default executor.
    pub fn new() -> Self {
        Self::new_with_executor(None)
    }

    /// Performs an HTTP request with the given method and optional body.
    async fn request<T>(
        &self,
        uri: T,
        method: Method,
        body: Option<Bytes>,
    ) -> Result<Response<IncomingBody>>
    where
        T: TryInto<Uri>,
        T::Error: Into<anyhow::Error>,
    {
        let uri = uri
            .try_into()
            .map_err(|e| anyhow::anyhow!("Invalid URI: {}", e.into()))?;

        let mut builder = Request::builder().method(method).uri(&uri);

        // Set appropriate headers
        if let Some(ref bytes) = body {
            if !bytes.is_empty() {
                builder = builder.header("Content-Length", bytes.len());
                builder = builder.header("Content-Type", "application/octet-stream");
            }
        }

        // Add HTTP/2 preference header
        builder = builder.header("Connection", "Upgrade, HTTP2-Settings");
        builder = builder.header("Upgrade", "h2c");

        let request = builder
            .body(match body {
                Some(bytes) => Full::new(bytes),
                None => Full::new(Bytes::new()),
            })
            .context("Failed to build request")?;

        self.client
            .request(request)
            .await
            .context(format!("Request to {} failed", uri))
    }

    /// Performs a simple GET request.
    pub async fn get<T>(&self, uri: T) -> Result<Response<IncomingBody>>
    where
        T: TryInto<Uri>,
        T::Error: Into<anyhow::Error>,
    {
        self.request(uri, Method::GET, None).await
    }

    /// Performs a simple POST request with a request body.
    pub async fn post<T>(&self, uri: T, body: Bytes) -> Result<Response<IncomingBody>>
    where
        T: TryInto<Uri>,
        T::Error: Into<anyhow::Error>,
    {
        self.request(uri, Method::POST, Some(body)).await
    }

    /// Performs a PUT request with a request body.
    pub async fn put<T>(&self, uri: T, body: Bytes) -> Result<Response<IncomingBody>>
    where
        T: TryInto<Uri>,
        T::Error: Into<anyhow::Error>,
    {
        self.request(uri, Method::PUT, Some(body)).await
    }

    /// Performs a DELETE request.
    pub async fn delete<T>(&self, uri: T) -> Result<Response<IncomingBody>>
    where
        T: TryInto<Uri>,
        T::Error: Into<anyhow::Error>,
    {
        self.request(uri, Method::DELETE, None).await
    }

    /// Performs a PATCH request with a request body.
    pub async fn patch<T>(&self, uri: T, body: Bytes) -> Result<Response<IncomingBody>>
    where
        T: TryInto<Uri>,
        T::Error: Into<anyhow::Error>,
    {
        self.request(uri, Method::PATCH, Some(body)).await
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use http_body_util::BodyExt;
    use hyper::StatusCode;

    use super::*;
    #[tokio::test]
    #[cfg(not(feature = "disable-online-tests"))]
    async fn test_get_request() {
        let client = HttpClient::new();

        let response = client
            .get("https://httpbin.org/get")
            .await
            .expect("Request failed");
        assert_eq!(response.status(), StatusCode::OK);

        let body_bytes = response
            .into_body()
            .collect()
            .await
            .expect("Failed to collect body")
            .to_bytes();

        let body_string = String::from_utf8_lossy(&body_bytes);
        assert!(body_string.contains("httpbin.org"));
    }

    #[tokio::test]
    #[cfg(not(feature = "disable-online-tests"))]
    async fn test_post_request() {
        let client = HttpClient::new();

        let body = Bytes::from("Test POST body");
        let response = client
            .post("https://httpbin.org/post", body.clone())
            .await
            .expect("Request failed");
        assert_eq!(response.status(), StatusCode::OK);

        let response_body = response
            .into_body()
            .collect()
            .await
            .expect("Failed to collect body")
            .to_bytes();

        let response_body_string = String::from_utf8_lossy(&response_body);
        assert!(response_body_string.contains("Test POST body"));
    }

    #[tokio::test]
    #[cfg(not(feature = "disable-online-tests"))]
    async fn test_put_request() {
        let client = HttpClient::new();

        let body = Bytes::from("Test PUT body");
        let response = client
            .put("https://httpbin.org/put", body.clone())
            .await
            .expect("Request failed");
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    #[cfg(not(feature = "disable-online-tests"))]
    async fn test_delete_request() {
        let client = HttpClient::new();

        let response = client
            .delete("https://httpbin.org/delete")
            .await
            .expect("Request failed");
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_invalid_uri() {
        let client = HttpClient::new();
        let result = client.get("invalid_uri").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    #[cfg(not(feature = "disable-online-tests"))]
    async fn test_client_clone() {
        let client = HttpClient::new();
        let cloned_client = client.clone();

        // Both clients should work independently
        let response1 = client
            .get("https://httpbin.org/get")
            .await
            .expect("Request failed");
        let response2 = cloned_client
            .get("https://httpbin.org/get")
            .await
            .expect("Request failed");

        assert_eq!(response1.status(), StatusCode::OK);
        assert_eq!(response2.status(), StatusCode::OK);
    }
}
