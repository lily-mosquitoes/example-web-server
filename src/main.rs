#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
extern crate dotenv;

use dotenv::dotenv;

pub mod connection;
pub mod routes;
pub mod repository;
pub mod models;
pub mod schema;

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build().attach(connection::DbConn::fairing()).mount("/", routes![routes::get_ifus, routes::get_ifu, routes::post_ifu, routes::update_ifu, routes::delete_ifu ])
}
