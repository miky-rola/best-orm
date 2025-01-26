use std::collections::HashMap;
use async_trait::async_trait;
use sea_query::Iden;
use crate::database::connection::{DatabaseConnection, OrmError};
use crate::database::query_builder::QueryBuilder;

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
    
    fn query(db: &DatabaseConnection) -> QueryBuilder<Self>;
    
    fn validate(&self) -> Result<(), OrmError>;
    fn table_name() -> String;
    fn primary_key() -> String;
}