//! Provides [`Query`] and [`Mutation`] for databases reads/writes.

mod migrate;
mod mutation;
mod query;

pub use migrate::migrate;
pub use mutation::Mutation;
pub use query::Query;
