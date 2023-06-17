use serenity::prelude::TypeMapKey;
use std::sync::Arc;
use tokio::sync::RwLock;
use sqlx::SqlitePool;
pub struct Database;

impl TypeMapKey for Database {
    type Value = Arc<RwLock<SqlitePool>>;
}