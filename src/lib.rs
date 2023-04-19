//! `sea_skipper` is a library that helps with [`sea-orm`] usage in some specific use cases.

// TODO: Add feature flags so these can be optionally enabled/disabled?
/// Contains trait for types that have a `Location` header.
mod location;
pub use location::*;

/// Traits for converting URL query strings into conditions for filtering database queries.
pub mod query_filter;

/// [`Resource`] and [`DataTrait`] encapsulating `sea-orm` types and trait bounds for simpler generics.
mod resource;
pub use resource::*;

mod orm;
pub use orm::*;

#[cfg(feature = "derive")]
pub use sea_skipper_macros::*;
