use sea_orm::IntoActiveModel;
use std::fmt::Debug;

use crate::Resource;

/// Captures trait bounds for a data model type associated with a [`Resource`].
//
// TODO: Consider whether this should have a Resource as an associated type instead of a generic.
#[allow(clippy::module_name_repetitions)]
pub trait DataTrait<R: Resource>: IntoActiveModel<R::ActiveModel> + Send + Clone + Debug {}

/// Blanket implementation for types that fit [`DataTrait`] type bounds.
//
// This was intended to be a type alias, but Rust does not currently support type aliases as
// associated types in trait definitions.
impl<T, R> DataTrait<R> for T
where
    T: IntoActiveModel<R::ActiveModel> + Send + Clone + Debug,
    R: Resource,
{
}
