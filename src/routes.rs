use rocket_sync_db_pools::diesel::result::Error;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use crate::connection::DbConn;
use crate::models;
use crate::repository;
use std::env;

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError,
    }
}

fn ifu_created(ifu: models::Ifu) -> status::Created<Json<models::Ifu>> {
    let host = env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set");
    let port = env::var("ROCKET_PORT").expect("ROCKET_PORT must be set");
    status::Created::new(
        format!("{host}:{port}/people/{id}", host = host, port = port, id = ifu.id).to_string()
    ).body(Json(ifu))
}

#[get("/")]
pub fn index() -> &'static str {
    "Works!"
}

#[get("/ifus")]
pub async fn get_ifus(connection: DbConn) -> Result<Json<Vec<models::Ifu>>, Status> {
    connection.run( |c| repository::all_ifus(c)
        .map(|ifus| Json(ifus))
        .map_err(|error| error_status(error))
    ).await
}

#[get("/ifu/<id>")]
pub async fn get_ifu(id: i32, connection: DbConn) -> Result<Json<models::Ifu>, Status> {
    connection.run( move |c| repository::get_ifu(id, c)
        .map(|ifu| Json(ifu))
        .map_err(|error| error_status(error))
    ).await
}

#[post("/ifu", format="application/json", data="<ifu>")]
pub async fn post_ifu(ifu: Json<models::Ifu>, connection: DbConn) -> Result<status::Created<Json<models::Ifu>>, Status> {
    connection.run( |c| repository::insert_ifu(ifu.into_inner(), c)
        .map(|ifu| ifu_created(ifu))
        .map_err(|error| error_status(error))
    ).await
}
