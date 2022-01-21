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

use crate::routes::ifus;
use crate::routes::products;

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build().attach(connection::DbConn::fairing()).mount("/", routes![ifus::all, ifus::get, ifus::post, ifus::update, ifus::delete, products::all, products::get, products::post, products::update, products::delete])
}
