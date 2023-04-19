//! The `query_filter` allows URL query strings to be converted into conditions for database queries.
//!
//! The current implementation is limited to mapping query string parameters to columns and supports exact equality comparisons only.
use std::collections::HashMap;
use std::fmt::Debug;

use sea_orm::{
    sea_query::{Condition, IntoCondition},
    ColumnTrait, EntityTrait,
};

// TODO: Add to example.

/// A trait describing how query string parameters map to a database entity's [`EntityTrait::Column`s](EntityTrait::Column).
pub trait QueryParams: Debug {
    // TODO: Resource or entity?
    /// Database `Entity` that `Self` is defined to filter on.
    type Entity: EntityTrait;

    /// Returns the `Column` filtered by `query_param` if it's a valid query filter key (otherwise `None`).
    fn column(query_param: &str) -> Option<<Self::Entity as EntityTrait>::Column>;
}

/// Stores [`Column`s](EntityTrait::Column) and value filters constructed from a request URL's query string.
///
/// [`QueryFilter`] represents valid filters (for some [`QueryParams`] definition) and may be constructed
/// via [`from_query_string`](Self::from_query_string) when handling a request.
///
/// [`Self::into_condition()`] may be used to convert this [`QueryFilter`] into a [`Condition`] for use
/// in database queries.
#[derive(Debug)]
pub struct QueryFilter<P: QueryParams>(Vec<(<P::Entity as EntityTrait>::Column, String)>);

impl<P: QueryParams + Debug> QueryFilter<P> {
    /// Constructs [`QueryFilter`] from a request URL's `query_string`.
    ///
    /// This will only store columns and values for query parameters mapped to columns in the
    /// [`QueryParams::column()`] implementation; other parameters in the query string are ignored.
    #[must_use]
    pub fn from_query_string(query_string: HashMap<String, String>) -> Self {
        Self(
            query_string
                .into_iter()
                .filter_map(|(param, value)| P::column(&param).map(|column| (column, value)))
                .collect(),
        )
    }
}

/// Allows [`QueryFilter`] to be converted into an `ALL` [`Condition`] for database queries.
///
/// This returns a database query [`Condition`] that tests equality of all column values with the
/// values from the query string.
impl<P: QueryParams> IntoCondition for QueryFilter<P> {
    fn into_condition(self) -> Condition {
        self.0
            .into_iter()
            .fold(Condition::all(), |all, (column, value)| {
                all.add(column.eq(value))
            })
    }
}
