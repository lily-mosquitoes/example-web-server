use rocket_sync_db_pools::{diesel, database};

#[database("db")]
pub struct DbConn(diesel::PgConnection);
