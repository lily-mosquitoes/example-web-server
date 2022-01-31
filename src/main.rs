#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
extern crate dotenv;

use dotenv::dotenv;
use rocket::Request;
use rocket_dyn_templates::Template;
use rocket::fs::{FileServer, relative};
use rocket::data::ToByteUnit;

pub mod connection;
pub mod routes;
pub mod repository;
pub mod models;
pub mod schema;

use crate::routes::{ifus, products, users, files, healthcheck, website};

#[catch(500)]
fn internal_server_error() -> String {
    "Internal Server Error / Operation Not Allowed".to_string()
}

#[catch(413)]
fn payload_too_large(req: &Request) -> String {
    println!("type: {:?}", req.format().unwrap().sub().to_string());
    format!(
        "Payload too large, limits: '{}'",
        req.limits().get(req.format()
            .unwrap_or(&rocket::http::MediaType::Any).sub().to_string()
        ).unwrap_or(0_usize.bytes())
    )
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Resource not found: '{}'", req.uri())
}

#[catch(403)]
fn forbidden() -> String {
    "Resource not available to requester".to_string()
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build()
        .attach(connection::DbConn::fairing())
        .attach(Template::fairing())
        .register("/", catchers![
            internal_server_error,
            payload_too_large,
            not_found,
            forbidden,
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
            healthcheck::ok,
            website::index,
            website::search,
        ])
}
