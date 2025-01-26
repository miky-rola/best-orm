// use std::collections::HashMap;
use sea_query::Iden;
use sqlx::{FromRow, postgres::PgRow};
use crate::database::connection::{DatabaseConnection, OrmError};
use crate::database::query_builder::QueryBuilder;

pub enum Condition {
    Equals(String, String),          
    NotEquals(String, String),       
    GreaterThan(String, String),    
    GreaterThanOrEqual(String, String), 
    LessThan(String, String),        
    LessThanOrEqual(String, String), 
    Like(String, String),            
    NotLike(String, String),         
    In(String, Vec<String>),        
    NotIn(String, Vec<String>),      
    IsNull(String),                  
    IsNotNull(String),               
    Between(String, String, String), 
    NotBetween(String, String, String), 
    And(Vec<Condition>),             
    Or(Vec<Condition>),             
}

pub trait Model: Sized + Send + Sync + for<'r> FromRow<'r, PgRow> {
    type Columns: Iden;
    
    fn find_by(db: &DatabaseConnection, conditions: Vec<Condition>) -> Result<Vec<Self>, OrmError>;
    fn find_one_by(db: &DatabaseConnection, conditions: Vec<Condition>) -> Result<Option<Self>, OrmError>;
    
    fn create(self, db: &DatabaseConnection) -> Result<Self, OrmError>;
    fn create_many(models: Vec<Self>, db: &DatabaseConnection) -> Result<Vec<Self>, OrmError>;
    
    fn update(&mut self, db: &DatabaseConnection) -> Result<(), OrmError>;
    fn update_by_id(&mut self, db: &DatabaseConnection, id: &str) -> Result<(), OrmError>;
    
    fn delete(&self, db: &DatabaseConnection) -> Result<(), OrmError>;
    fn delete_by_id(db: &DatabaseConnection, id: &str) -> Result<(), OrmError>;
    
    fn query(db: &DatabaseConnection) -> QueryBuilder<Self>;
    
    fn validate(&self) -> Result<(), OrmError>;
    fn table_name() -> String;
    fn primary_key() -> String;
}