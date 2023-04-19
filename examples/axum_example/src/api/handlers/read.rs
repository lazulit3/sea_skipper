//! Request [handlers][axum::handler] for `GET` requests.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::DatabaseConnection;
use sea_skipper::Resource;

use crate::db::Query;

/// Handles requests getting a resource by ID.
///
/// # Errors
/// Responds with `404 Not Found` status when no resource matching `id` is found in the database.
/// Responds with `500 Internal Server Error` status when the database query fails.
pub async fn get_by_id<R: Resource>(
    State(db): State<DatabaseConnection>,
    Path(id): Path<R::Id>,
) -> Result<Json<R::Data>, StatusCode>
where
    R: Resource,
{
    let result = Query::find_by_id::<R, R::Id>(&db, id).await;
    match result {
        Ok(Some(data)) => Ok(Json(data)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Handles requests getting a collection of [`Resource`] objects.
///
/// # Errors
/// Responds with `500 Internal Server Error` status when the database query produces an error.
pub async fn get_collection<R: Resource>(
    State(db): State<DatabaseConnection>,
) -> Result<Json<Vec<R::Data>>, StatusCode> {
    Ok(Json(
        Query::find_all::<R>(&db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    ))
}
