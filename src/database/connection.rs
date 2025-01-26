use sqlx::{mysql::MySqlPool, postgres::PgPool};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OrmError {
    #[error("Database connection error: {0}")]
    ConnectionError(String),
    #[error("Query execution error: {0}")]
    QueryError(String),
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Not found: {0}")]
    NotFound(String),
}

#[derive(Debug, Clone)]
pub enum DatabaseType {
    Postgres(PgPool),
    MySql(MySqlPool),
}

pub struct DatabaseConnection {
    pub connection: DatabaseType,
}

impl DatabaseConnection {
    pub async fn new(db_type: DatabaseType, connection_string: &str) -> Result<Self, OrmError> {
        match db_type {
            DatabaseType::Postgres(_)  => {
                let pool = PgPool::connect(connection_string)
                    .await
                    .map_err(|e| OrmError::ConnectionError(e.to_string()))?;
                Ok(Self { connection: DatabaseType::Postgres(pool) })
            },
            DatabaseType::MySql(_) => {
                let pool = MySqlPool::connect(connection_string)
                    .await
                    .map_err(|e| OrmError::ConnectionError(e.to_string()))?;
                Ok(Self { connection: DatabaseType::MySql(pool) })
            },
        }
    }
}