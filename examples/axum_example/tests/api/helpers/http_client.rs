//! `http_client` provides a wrapper around `reqwest::Client` that accepts relative URLs for tests.
//!
//! This is a modified version of `axum-test-helper` with the service initialization
//! removed.

use axum::http::{
    self,
    header::{HeaderName, HeaderValue},
    StatusCode,
};
use bytes::Bytes;
use std::convert::TryFrom;

pub struct Client {
    client: reqwest::Client,
    service_url: String,
}

impl Client {
    pub fn new(service_url: &str) -> Client
where {
        let client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .unwrap();

        Client {
            client,
            service_url: service_url.to_string(),
        }
    }

    #[allow(dead_code)]
    pub fn get(&self, url: &str) -> RequestBuilder {
        RequestBuilder {
            builder: self.client.get(format!("{}{}", self.service_url, url)),
        }
    }

    #[allow(dead_code)]
    pub fn head(&self, url: &str) -> RequestBuilder {
        RequestBuilder {
            builder: self.client.head(format!("{}{}", self.service_url, url)),
        }
    }

    #[allow(dead_code)]
    pub fn post(&self, url: &str) -> RequestBuilder {
        RequestBuilder {
            builder: self.client.post(format!("{}{}", self.service_url, url)),
        }
    }

    #[allow(dead_code)]
    pub fn put(&self, url: &str) -> RequestBuilder {
        RequestBuilder {
            builder: self.client.put(format!("{}{}", self.service_url, url)),
        }
    }

    #[allow(dead_code)]
    pub fn patch(&self, url: &str) -> RequestBuilder {
        RequestBuilder {
            builder: self.client.patch(format!("{}{}", self.service_url, url)),
        }
    }

    #[allow(dead_code)]
    pub fn delete(&self, url: &str) -> RequestBuilder {
        RequestBuilder {
            builder: self.client.delete(format!("{}{}", self.service_url, url)),
        }
    }
}

#[allow(dead_code)]
pub struct RequestBuilder {
    builder: reqwest::RequestBuilder,
}

impl RequestBuilder {
    #[allow(dead_code)]
    pub async fn send(self) -> TestResponse {
        TestResponse {
            response: self.builder.send().await.unwrap(),
        }
    }

    #[allow(dead_code)]
    pub fn body(mut self, body: impl Into<reqwest::Body>) -> Self {
        self.builder = self.builder.body(body);
        self
    }

    #[allow(dead_code)]
    pub fn json<T>(mut self, json: &T) -> Self
    where
        T: serde::Serialize,
    {
        self.builder = self.builder.json(json);
        self
    }

    #[allow(dead_code)]
    pub fn header<K, V>(mut self, key: K, value: V) -> Self
    where
        HeaderName: TryFrom<K>,
        <HeaderName as TryFrom<K>>::Error: Into<http::Error>,
        HeaderValue: TryFrom<V>,
        <HeaderValue as TryFrom<V>>::Error: Into<http::Error>,
    {
        self.builder = self.builder.header(key, value);
        self
    }
}

#[allow(dead_code)]
pub struct TestResponse {
    response: reqwest::Response,
}

impl TestResponse {
    #[allow(dead_code)]
    pub async fn text(self) -> String {
        self.response.text().await.unwrap()
    }

    #[allow(dead_code)]
    pub async fn json<T>(self) -> T
    where
        T: serde::de::DeserializeOwned,
    {
        self.response.json().await.unwrap()
    }

    #[allow(dead_code)]
    pub fn status(&self) -> StatusCode {
        self.response.status()
    }

    #[allow(dead_code)]
    pub fn headers(&self) -> &http::HeaderMap {
        self.response.headers()
    }

    #[allow(dead_code)]
    pub async fn chunk(&mut self) -> Option<Bytes> {
        self.response.chunk().await.unwrap()
    }

    #[allow(dead_code)]
    pub async fn chunk_text(&mut self) -> Option<String> {
        let chunk = self.chunk().await?;
        Some(String::from_utf8(chunk.to_vec()).unwrap())
    }
}
