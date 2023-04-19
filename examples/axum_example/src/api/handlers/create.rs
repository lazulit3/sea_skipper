use axum::{
    extract::{Json, State},
    http::{header, StatusCode},
    response::IntoResponse,
};
use sea_orm::DatabaseConnection;
use sea_skipper::{DataTrait, Location, Resource};
use serde::Serialize;

use crate::db::Mutation;

/// Handles requests to create a new resource with JSON data from the request body.
///
/// # Response
/// Responds with `201 Created` status and the created resource's `Location` header.
///
/// # Errors
/// Responds with `500 Internal Server Error` when the database mutation fails.
pub async fn create<R, D>(
    State(db): State<DatabaseConnection>,
    Json(data): Json<D>,
) -> Result<impl IntoResponse, StatusCode>
where
    R: Resource,
    R::Data: Location,
    D: DataTrait<R>,
{
    // TODO: Update create request example to demo implementation that catches a unique constraint
    // violation for unique fields and uses `NewModel`'s `ModelCondition::to_all_condition()` to
    // check if the failed post request would have been idempotent (i.e. data matcing `NewModel` in
    // the database.) If so return a `303 See Other` redirect versus `409 Conflict`.
    let result = Mutation::create(&db, data).await;
    match result {
        Ok(data) => Ok(created(data)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Returns a created response for created [`Data`].
pub fn created<D>(data: D) -> impl IntoResponse
where
    D: Location + Serialize,
{
    (
        StatusCode::CREATED,
        [(header::LOCATION, data.location())],
        Json(data),
    )
}
