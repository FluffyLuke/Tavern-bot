use serenity::prelude::TypeMapKey;
use std::sync::Arc;
use tokio::sync::RwLock;
use sqlx::SqlitePool;
pub struct Database;

impl TypeMapKey for Database {
    type Value = Arc<RwLock<SqlitePool>>;
}

#[macro_export]
macro_rules! ctx_get_lock {
    ($ctx:expr, $type:ty, Mode::Write) => {{
        let data_read = $ctx.data.write().await;
        let quote_lock = data_read.get::<$type>().expect("Cannot get the lock");
        quote_lock.write().await
    }};
    ($ctx:expr, $type:ty, Mode::Read) => {{
        let data_read = $ctx.data.read().await;
        let quote_lock = data_read.get::<$type>().expect("Cannot get the lock");
        quote_lock.read().await
    }};
}

pub enum Mode {
    Write,
    Read,
}