#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
extern crate dotenv;
extern crate rocket_multipart_form_data;

use dotenv::dotenv;
use rocket::Request;
use rocket_dyn_templates::Template;
use rocket::fs::{FileServer, relative};

pub mod connection;
pub mod routes;
pub mod repository;
pub mod models;
pub mod schema;

use crate::routes::{ifus, products, users, files, website};

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
        .attach(Template::fairing())
        .register("/", catchers![
            internal_server_error,
            forbidden,
            not_found,
        ])
        .mount("/", FileServer::from(relative!("static")))
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
            website::index,
        ])
}
