//! Helpers for API integration tests.

/// HTTP client wrapper around `reqwest::Client` for relative paths in URLs.
pub mod http_client;
/// `TestService` for API service & databases isolated per test.
pub mod service;
