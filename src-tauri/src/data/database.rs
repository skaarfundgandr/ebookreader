use std::{env, sync::Arc};

use diesel::SqliteConnection;
use diesel_async::{
    pooled_connection::{
        deadpool::{Object, Pool, PoolError, Hook, HookError},
        AsyncDieselConnectionManager,
    },
    sync_connection_wrapper::SyncConnectionWrapper,
    SimpleAsyncConnection,
};
use dotenvy::dotenv;
use once_cell::sync::Lazy;
use tokio::sync::Mutex;

/// Returns a connection from the database connection pool
///
/// # Usage
///
/// ```rust,ignore
/// // Create a database connection
/// let conn = connect_from_pool().await;
/// // Handle errors(if any)
/// let mut conn = match conn {
///     Ok(value) => value,
///     Err(e) => panic!("Failed to connect from pool: {e}"),
/// };
/// // Use the connection
/// let res = books
///     .select(Books::as_select())
///     .load(&mut conn)
///     .await;
/// // Handle errors on results
/// let results = match res {
///     Ok(value) => value,
///     Err(e) => panic!("Failed to fetch results: {e}"),
/// };
/// ```
pub async fn connect_from_pool(
) -> Result<Object<SyncConnectionWrapper<SqliteConnection>>, PoolError> {
    return DB_POOL.get().await;
}
/// Lazily initializes a database connection pool
static DB_POOL: Lazy<Pool<SyncConnectionWrapper<SqliteConnection>>> = Lazy::new(|| {
    dotenv().ok();

    let database_path = env::var("DATABASE_URL").expect("Failed to find database URL from env!");
    let config =
        AsyncDieselConnectionManager::<SyncConnectionWrapper<SqliteConnection>>::new(database_path);

    Pool::builder(config)
        .post_create(Hook::async_fn(|conn: &mut SyncConnectionWrapper<SqliteConnection>, _meta| Box::pin(async move {
            conn.batch_execute(
                "
                PRAGMA foreign_keys = ON;
                PRAGMA journal_mode = WAL;
                PRAGMA synchronous = NORMAL;
                PRAGMA temp_store = MEMORY;
                PRAGMA mmap_size = 30000000000;
                PRAGMA busy_timeout = 5000;
                ",
            )
            .await
            .map_err(|e| HookError::Message(format!("Failed to set SQLite pragmas: {e}").into()))
        })))
        .build()
        .expect("Failed to create SQLite connection pool")
});

pub fn lock_db() -> Arc<Mutex<()>> {
    return DB_LOCK.clone();
}

static DB_LOCK: Lazy<Arc<Mutex<()>>> = Lazy::new(|| Arc::new(Mutex::new(())));
