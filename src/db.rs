use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use dotenv::dotenv;
use std::env;

use crate::error::Result;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;
/// Initializes the database connection pool
pub fn initialize() -> PgPool {
    dotenv().ok();

    let conn_string = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(&conn_string);

    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    pool
}

/// Handles the creation of connections from the PgPool
pub fn pool_handler(pool: PgPool) -> Result<PgPooledConnection> {
    Ok(pool.get()?)
}
