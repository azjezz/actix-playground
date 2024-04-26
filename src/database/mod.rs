pub mod action;
pub mod model;
pub mod schema;

use diesel::{prelude::*, r2d2};

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;
pub type Pool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

/// Initialize database connection pool based on `DATABASE_URL` environment variable.
pub fn initialize_db_pool() -> Pool {
    let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL should be set");
    let manager = r2d2::ConnectionManager::<SqliteConnection>::new(conn_spec);

    r2d2::Pool::builder()
        .build(manager)
        .expect("database URL should be valid path to SQLite DB file")
}
