//! Provides [`Mutation`] for modifying entity data in the database.
//!
//! [`Mutation`] methods are generic over [`sea_orm`] entities (implementing [`EntityTrait`]) so they
//! can be re-used for different types of data.

use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, DbConn, DbErr, DeleteResult, EntityTrait,
    IntoActiveModel, PrimaryKeyTrait,
};

/// Creates, updates, and deletes entity data in the database.
pub struct Mutation;

impl Mutation {
    /// Inserts an entity into the database.
    pub async fn create<E, D, A>(db: &DbConn, new_data: D) -> Result<E::Model, DbErr>
    where
        E: EntityTrait,
        E::Model: IntoActiveModel<A>,
        D: IntoActiveModel<A>,
        A: ActiveModelTrait<Entity = E> + ActiveModelBehavior + Send,
    {
        new_data.into_active_model().insert(db).await
    }

    /// Deletes an entity from the database matching `id`.
    pub async fn delete_by_id<E: EntityTrait, Id>(
        db: &DbConn,
        id: Id,
    ) -> Result<DeleteResult, DbErr>
    where
        Id: Into<<E::PrimaryKey as PrimaryKeyTrait>::ValueType>,
    {
        <E as EntityTrait>::delete_by_id(id).exec(db).await
    }
}
