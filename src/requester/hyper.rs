use std::collections::HashMap;
use hyper_rustls::{HttpsConnectorBuilder, HttpsConnector};
use hyper::{Request, Body, Method};
use hyper::client::HttpConnector;
use async_trait::async_trait;
use crate::{Result, Requester};

pub struct HyperRustlsRequester (hyper::Client<HttpsConnector<HttpConnector>, Body>);

#[cfg(feature = "hyper-rustls")]
#[async_trait]
impl Requester for HyperRustlsRequester {
    async fn request(&self, url: String, body: String, headers: HashMap<&str, &str>) -> Result<Vec<u8>> {
        let mut req = Request::builder()
            .method(Method::POST)
            .uri(url);

        for (k, v) in headers {
            req = req.header(k, v);
        }

        let req = req
            .body(Body::from(body))
            .unwrap();

        let (_parts, resp_body) = self.0.request(req).await?.into_parts();
        let bytes = hyper::body::to_bytes(resp_body).await?;
        Ok(bytes.to_vec())
    }
}

impl HyperRustlsRequester {
    pub fn new() -> Self {
        let https = HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_or_http()
            .enable_http1()
            .build();
        let client = hyper::Client::builder().build(https);
        Self (client)
    }
}
