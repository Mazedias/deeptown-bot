pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};

use crate::bot::DbPool;

pub fn establish_connection(database_url: &str) -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}