use std::collections::HashMap;
use sqlx::{mysql::MySqlPool, postgres::PgPool};
use async_trait::async_trait;
use sea_query::{Iden, Condition, Expr};
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
    pub async fn new(db_type: &str, connection_string: &str) -> Result<Self, OrmError> {
        match db_type {
            "postgres" => {
                let pool = PgPool::connect(connection_string)
                    .await
                    .map_err(|e| OrmError::ConnectionError(e.to_string()))?;
                Ok(Self { connection: DatabaseType::Postgres(pool) })
            },
            "mysql" => {
                let pool = MySqlPool::connect(connection_string)
                    .await
                    .map_err(|e| OrmError::ConnectionError(e.to_string()))?;
                Ok(Self { connection: DatabaseType::MySql(pool) })
            },
            _ => Err(OrmError::ConnectionError("Unsupported database type".to_string()))
        }
    }
}

#[async_trait]
pub trait Model: Sized + Send + Sync {
    type Columns: Iden;
    
    async fn find_by(db: &DatabaseConnection, conditions: &HashMap<String, String>) -> Result<Vec<Self>, OrmError>;
    async fn find_one_by(db: &DatabaseConnection, conditions: &HashMap<String, String>) -> Result<Option<Self>, OrmError>;
    
    async fn create(self, db: &DatabaseConnection) -> Result<Self, OrmError>;
    async fn create_many(models: Vec<Self>, db: &DatabaseConnection) -> Result<Vec<Self>, OrmError>;
    
    async fn update(&mut self, db: &DatabaseConnection) -> Result<(), OrmError>;
    async fn update_by_id(&mut self, db: &DatabaseConnection, id: &str) -> Result<(), OrmError>;
    
    async fn delete(&self, db: &DatabaseConnection) -> Result<(), OrmError>;
    async fn delete_by_id(db: &DatabaseConnection, id: &str) -> Result<(), OrmError>;
    
    async fn query(db: &DatabaseConnection) -> QueryBuilder<Self>;
    
    fn validate(&self) -> Result<(), OrmError>;
    fn table_name() -> String;
    fn primary_key() -> String;
}

pub struct QueryBuilder<T: Model> {
    conditions: Vec<Condition>,
    limit: Option<u64>,
    offset: Option<u64>,
    order_by: Option<(String, bool)>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: Model> QueryBuilder<T> {
    pub fn where_eq(mut self, column: &str, value: &str) -> Self {
        self.conditions.push(Expr::col(column).eq(value));
        self
    }
    
    pub fn limit(mut self, limit: u64) -> Self {
        self.limit = Some(limit);
        self
    }
    
    pub fn offset(mut self, offset: u64) -> Self {
        self.offset = Some(offset);
        self
    }
    
    pub fn order_by(mut self, column: &str, descending: bool) -> Self {
        self.order_by = Some((column.to_string(), descending));
        self
    }
    
    pub async fn execute(self, _db: &DatabaseConnection) -> Result<Vec<T>, OrmError> {
        unimplemented!()
    }
}

fn main() {
    println!("Hello, world!");
}