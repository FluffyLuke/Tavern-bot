use serenity::prelude::TypeMapKey;
use std::sync::Arc;
use tokio::sync::RwLock;
use sqlx::SqlitePool;
pub struct Database;

impl TypeMapKey for Database {
    type Value = Arc<RwLock<SqlitePool>>;
}

#[macro_export]
macro_rules! guild_inited {
    ($ctx:expr, $id:expr) => {{
        let data_read = $ctx.data.read().await;
        let lock = data_read.get::<Database>().expect("Cannot get the lock");
        let database = lock.read().await;
        let query = sqlx::query!("Select * from guild where guild_id = ?", $id)
            .fetch_one(&*database)
            .await;
        if let Err(_) = query { 
            sqlx::query!("Insert into guild (guild_id) values (?)", $id)
                .execute(&*database)
                .await?;
        }
    }};
}