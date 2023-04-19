use sea_orm::{Condition, ModelTrait};

/// Trait for converting a data model into a [`Condition`] for database queries.
///
/// [`to_all_condition()`] returns a `Condition` that selects database records whose values are
/// equal to all of `Self`'s values.
///
/// This is useful when working with `NewModel` structs (derived with [`DeriveNewModel`]) that
/// do not have fields containing a primary key ID. (With a regular `Model` you would just query
/// by the model's unique ID.)
///
/// Consider a use case where a `NewModel` contains a unique field (that is not part of the primary key)
/// and a create (`POST`) request results in a unique constraint violation error. An HTTP API may want
/// to determine whether a record matching `NewModel`'s values already exists in the database to
/// decide between returning a `303 See Other` redirect versus a `409 Conflict` error.
pub trait ModelCondition: ModelTrait {
    /// Returns an `ALL` [`Condition`] that filters for column values equal to `Self`'s values.
    // TODO: Document an example of a model and the `Condition` this produces.
    fn to_all_condition(self) -> Condition;
}
