use std::env;

use diesel::SqliteConnection;
use diesel_async:: {
    pooled_connection:: {
    deadpool::{Object, Pool, PoolError}, AsyncDieselConnectionManager 
}, sync_connection_wrapper::SyncConnectionWrapper
};
use dotenvy::dotenv;
use once_cell::sync::Lazy;

pub async fn connect_from_pool() -> Result<Object<SyncConnectionWrapper<SqliteConnection>>, PoolError> {
    return DB_POOL.get().await;
}

static DB_POOL: Lazy<Pool<SyncConnectionWrapper<SqliteConnection>>> = Lazy::new(|| {
    dotenv().ok();

    let database_path = env::var("DATABASE_URL").expect("Failed to find database URL from env!");
    let config = AsyncDieselConnectionManager::<SyncConnectionWrapper<SqliteConnection>>::new(database_path);

    Pool::builder(config)
        .build()
        .expect("Failed to create SQLite connection pool")
});