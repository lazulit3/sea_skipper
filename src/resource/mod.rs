//! Traits describing API [`Resource`]s and [`data`].
mod data;

use std::fmt::Debug;

use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, EntityTrait, IntoActiveModel, ModelTrait,
    PrimaryKeyTrait,
};
use serde::Serialize;

pub use crate::Location;
pub use data::*;

/// A type that encapsulates `sea-orm` entity types and captures their trait bounds.
///
/// This may be used to simplify the trait bounds when passing `sea-orm` model types as generics in
/// functions that perform database operations using `sea-orm`.
pub trait Resource: EntityTrait<Model = Self::Data> {
    /// The `ActiveModel` type from the `sea-orm` entity of the [`Resource`].
    type ActiveModel: ActiveModelTrait<Entity = Self> + ActiveModelBehavior + Send;

    /// The `Model` type from the `sea-orm` entity of the [`Resource`].
    type Data: ModelTrait<Entity = Self>
        + IntoActiveModel<Self::ActiveModel>
        + Clone
        + Send
        + Serialize;

    /// Captures type of [`Data`] / [`EntityTrait::Model`]'s primary key.
    type Id: Into<<Self::PrimaryKey as PrimaryKeyTrait>::ValueType> + Debug;
}
