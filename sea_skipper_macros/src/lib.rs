extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Error};

mod attributes;
mod derives;

/// Implements [`ModelCondition`] on a type implementing `ModelTrait`.
// TODO: Add an example and some description of [`ModelCondition`].
#[proc_macro_derive(DeriveModelCondition, attributes(sea_orm))]
pub fn derive_model_condition(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ts: TokenStream = derives::expand_derive_model_condition(input)
        .unwrap_or_else(Error::into_compile_error)
        .into();
    ts
}

/// Derives a `NewModel` type from a `Model` and excludes the model's primary key fields.
///
/// A `NewModel` is equivalent to a `Model` after removing all fields containing the
/// `#[sea_orm(primary_key)]` attribute.
///
/// [`DeriveNewModel`] implements [`ModelCondition`] on the `NewModel` type.
///
/// This does not carry over field attributes from the original `Model` to the `NewModel`, so
/// additional derives on the `NewModel` that require `sea_orm` attributes may not work as
/// expected. (If you would like this functionality, please open an issue.)
///
/// # Usage
///
/// To derive a `NewModel` from a `Model`, add `DeriveNewModel` to the
/// `Model`'s derives:
///
// TODO: Update these examples so that they compile.
/// ```ignore
/// #[sea_orm(table_name = "posts")]
/// #[derive(
///     Clone, Debug, PartialEq, Eq, DeriveEntityModel, DeriveNewModel, Serialize, Deserialize,
/// )]
/// pub struct Model {
///     #[sea_orm(primary_key)]
///     #[serde(skip_deserializing)]
///     pub id: i32,
///     pub title: String,
///     #[sea_orm(column_type = "Text")]
///     pub text: String,
/// }
/// ```
///
/// This expands to a `NewModel` behind the scenes:
///
/// ```ignore
/// #[derive(
///     Clone, Debug, PartialEq, Eq, DeriveNewModel, Serialize, Deserialize,
/// )]
/// pub struct Model {
///     pub title: String,
///     pub text: String,
/// }
/// ````
#[proc_macro_derive(DeriveNewModel, attributes(sea_orm))]
pub fn derive_new_model(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ts: TokenStream = derives::expand_derive_new_model(input)
        .unwrap_or_else(Error::into_compile_error)
        .into();
    ts
}
