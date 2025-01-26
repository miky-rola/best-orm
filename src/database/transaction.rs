use sqlx::{Transaction, Postgres};
use crate::database::connection::{DatabaseConnection, OrmError};
use crate::database::connection::DatabaseType;

pub async fn begin_transaction(db: &DatabaseConnection) -> Result<Transaction<Postgres>, OrmError> {
    match &db.connection {
        DatabaseType::Postgres(pool) => {
            pool.begin().await.map_err(|e| OrmError::ConnectionError(e.to_string()))
        },
        _ => Err(OrmError::ConnectionError("Transaction not supported for this database type".to_string()))
    }
}

pub async fn commit_transaction(transaction: Transaction<'_, Postgres>) -> Result<(), OrmError> {
    transaction.commit().await.map_err(|e| OrmError::ConnectionError(e.to_string()))
}

pub async fn rollback_transaction(transaction: Transaction<'_,Postgres>) -> Result<(), OrmError> {
    transaction.rollback().await.map_err(|e| OrmError::ConnectionError(e.to_string()))
}