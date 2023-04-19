//! Provides [`Query`] for reading entity data from the database.
//!
//! [`Query`] methods are generic over [`sea_orm`] entities (implementing [`EntityTrait`]) so they
//! can be re-used for different types of data.
use sea_orm::{DbConn, DbErr, EntityTrait, PrimaryKeyTrait};

/// Reads entity data from the database.
pub struct Query;

/// Creates, updates, and deletes entity data in the database.
impl Query {
    /// Find all instances of an entity in a database.
    pub async fn find_all<E>(db: &DbConn) -> Result<Vec<<E as EntityTrait>::Model>, DbErr>
    where
        E: EntityTrait,
    {
        <E as EntityTrait>::find().all(db).await
    }

    /// Find an entity in the database by ID.
    pub async fn find_by_id<E, Id>(
        db: &DbConn,
        id: Id,
    ) -> Result<Option<<E as EntityTrait>::Model>, DbErr>
    where
        E: EntityTrait,
        Id: Into<<E::PrimaryKey as PrimaryKeyTrait>::ValueType>,
    {
        <E as EntityTrait>::find_by_id(id).one(db).await
    }
}
