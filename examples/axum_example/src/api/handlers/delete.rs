//! Request [handlers][axum::handler] for `DELETE` requests.

use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use sea_orm::{DatabaseConnection, DeleteResult};
use sea_skipper::Resource;

use crate::db::Mutation;

/// Handles requests to delete a resource matching `id`.
pub async fn delete_by_id<R: Resource>(
    State(db): State<DatabaseConnection>,
    Path(id): Path<R::Id>,
) -> Result<StatusCode, StatusCode> {
    let result = Mutation::delete_by_id::<R, R::Id>(&db, id).await;
    match result {
        Ok(DeleteResult { rows_affected: 0 }) => Err(StatusCode::NOT_FOUND),
        Ok(DeleteResult { rows_affected: 1.. }) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
