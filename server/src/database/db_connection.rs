use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::PgConnection;
use std::time::Duration;
use log;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn create_connection_pool(database_url: &str) -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    
    Pool::builder()
        .max_size(10)
        .min_idle(Some(5))
        .connection_timeout(Duration::from_secs(30))
        .idle_timeout(Some(Duration::from_secs(300)))
        .build(manager)
        .expect("Failed to create connection pool")
}

pub fn get_connection(pool: &DbPool) -> Result<DbConnection, r2d2::Error> {
    pool.get().map_err(|e| {
        log::error!("Failed to get database connection: {}", e);
        e
    })
} 