#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
extern crate dotenv;
extern crate rocket_multipart_form_data;

use dotenv::dotenv;
use rocket::Request;

pub mod connection;
pub mod routes;
pub mod repository;
pub mod models;
pub mod schema;

use crate::routes::ifus;
use crate::routes::products;
use crate::routes::users;
use crate::routes::files;

#[catch(500)]
fn internal_server_error() -> String {
    "Internal Server Error / Operation Not Allowed".to_string()
}

#[catch(403)]
fn forbidden() -> String {
    "Resource not available to requester".to_string()
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Resource not found: '{}'", req.uri())
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build()
        .attach(connection::DbConn::fairing())
        .register("/", catchers![
            internal_server_error, forbidden, not_found,
        ])
        .mount("/", routes![
            ifus::all,
            ifus::get,
            ifus::post,
            ifus::update,
            ifus::delete,
            products::all,
            products::get,
            products::post,
            products::update,
            products::delete,
            users::login,
            users::user_id,
            users::logout,
            users::all,
            users::get,
            users::post,
            users::update,
            users::make_admin,
            users::delete,
            files::all,
            files::get,
            files::download,
            files::post,
            files::delete,
        ])
}
