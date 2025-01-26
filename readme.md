# BestORM

BestORM is a lightweight and flexible Object-Relational Mapping (ORM) library for Rust, designed to simplify database interactions with support for PostgreSQL and MySQL. It provides a simple API for querying, inserting, updating, and deleting records, as well as managing transactions.

## Features

- **Database Support**: Works with PostgreSQL and MySQL.
- **Query Building**: Easily build complex SQL queries using a fluent API.
- **Transactions**: Supports database transactions for atomic operations.
- **Model Validation**: Built-in support for model validation.
- **Async/Await**: Fully asynchronous using `async`/`await`.

## Installation

Add the following dependencies to your `Cargo.toml` file:

```toml
[dependencies]
sqlx = { version = "0.7", features = ["postgres", "mysql", "runtime-tokio-native-tls"] }
sea-query = "0.28"
thiserror = "1.0"
async-trait = "0.1"
```

## Usage

### 1. Define Your Model

Create a model by implementing the `Model` trait. For example:

```rust
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use crate::models::base_model::Model;
use sea_query::Iden;

#[derive(Serialize, Deserialize, FromRow)]
struct User {
    id: i32,
    name: String,
    email: String,
}

#[async_trait::async_trait]
impl Model for User {
    type Columns = UserColumns;

    async fn find_by(db: &DatabaseConnection, conditions: &HashMap<String, String>) -> Result<Vec<Self>, OrmError> {
        // Implementation here
    }

    async fn find_one_by(db: &DatabaseConnection, conditions: &HashMap<String, String>) -> Result<Option<Self>, OrmError> {
        // Implementation here
    }

    async fn create(self, db: &DatabaseConnection) -> Result<Self, OrmError> {
        // Implementation here
    }

    async fn create_many(models: Vec<Self>, db: &DatabaseConnection) -> Result<Vec<Self>, OrmError> {
        // Implementation here
    }

    async fn update(&mut self, db: &DatabaseConnection) -> Result<(), OrmError> {
        // Implementation here
    }

    async fn update_by_id(&mut self, db: &DatabaseConnection, id: &str) -> Result<(), OrmError> {
        // Implementation here
    }

    async fn delete(&self, db: &DatabaseConnection) -> Result<(), OrmError> {
        // Implementation here
    }

    async fn delete_by_id(db: &DatabaseConnection, id: &str) -> Result<(), OrmError> {
        // Implementation here
    }

    fn query(db: &DatabaseConnection) -> QueryBuilder<Self> {
        QueryBuilder::new()
    }

    fn validate(&self) -> Result<(), OrmError> {
        // Implementation here
    }

    fn table_name() -> String {
        "users".to_string()
    }

    fn primary_key() -> String {
        "id".to_string()
    }
}

#[derive(Iden)]
enum UserColumns {
    Id,
    Name,
    Email,
}
```

### 2. Connect to the Database

Use the `DatabaseConnection` struct to connect to your database:

```rust
use crate::database::connection::DatabaseConnection;

#[tokio::main]
async fn main() {
    let db = DatabaseConnection::new("postgres", "postgres://user:password@localhost/dbname")
        .await
        .expect("Failed to connect to database");

    // Use the database connection
}
```

### 3. Query the Database

Use the `QueryBuilder` to build and execute queries:

```rust
let users = User::query(&db)
    .where_eq("name", "John")
    .limit(10)
    .execute(&db)
    .await
    .expect("Failed to fetch users");

for user in users {
    println!("User: {:?}", user);
}
```

### 4. Transactions

Use the `transaction` module to manage transactions:

```rust
use crate::database::transaction::{begin_transaction, commit_transaction};

let mut transaction = begin_transaction(&db)
    .await
    .expect("Failed to begin transaction");

// Perform database operations within the transaction

commit_transaction(transaction)
    .await
    .expect("Failed to commit transaction");
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.
