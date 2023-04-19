use axum::{routing::get, Router};
use sea_orm::DatabaseConnection;

use super::handlers::{
    create::create,
    delete::delete_by_id,
    read::{get_by_id, get_collection},
};
use crate::entity::cake;
use crate::entity::prelude::*;

/// Returns API ['Router'] that routes requests to handlers.
pub fn router(db: DatabaseConnection) -> Router {
    // TODO: Add update
    Router::new()
        .route(
            "/cakes",
            get(get_collection::<Cake>).post(create::<Cake, cake::NewModel>),
        )
        .route(
            "/cakes/:id",
            get(get_by_id::<Cake>).delete(delete_by_id::<Cake>),
        )
        .with_state(db)
}
