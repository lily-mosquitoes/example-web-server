#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate dotenv;

use dotenv::dotenv;
// use rocket_sync_db_pools::{diesel, database};

pub mod connection;
pub mod routes;

// #[database("db")]
// struct DbConn(diesel::PgConnection);

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build().attach(connection::DbConn::fairing()).mount("/", routes![routes::index])
}
