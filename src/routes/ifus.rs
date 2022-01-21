use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use crate::connection::DbConn;
use crate::models::ifus::Ifu;
use crate::repository::ifus;
use crate::routes::utils::{ error_status, record_created };

#[get("/ifus")]
pub async fn all(connection: DbConn) -> Result<Json<Vec<Ifu>>, Status> {
    connection.run( |c| ifus::all(c)
        .map(|ifus| Json(ifus))
        .map_err(|error| error_status(error))
    ).await
}

#[get("/ifu/<id>")]
pub async fn get(id: i32, connection: DbConn) -> Result<Json<Ifu>, Status> {
    connection.run( move |c| ifus::get(id, c)
        .map(|ifu| Json(ifu))
        .map_err(|error| error_status(error))
    ).await
}

#[post("/ifu", format="application/json", data="<ifu>")]
pub async fn post(ifu: Json<Ifu>, connection: DbConn) -> Result<status::Created<Json<Ifu>>, Status> {
    connection.run( |c| ifus::insert(ifu.into_inner(), c)
        .map(|ifu| record_created(ifu))
        .map_err(|error| error_status(error))
    ).await
}

#[put("/ifu/<id>", format="application/json", data="<ifu>")]
pub async fn update(id: i32, ifu: Json<Ifu>, connection: DbConn) -> Result<Json<Ifu>, Status> {
    connection.run( move |c| ifus::update(id, ifu.into_inner(), c)
        .map(|ifu| Json(ifu))
        .map_err(|error| error_status(error))
    ).await
}

#[delete("/ifu/<id>")]
pub async fn delete(id: i32, connection: DbConn) -> Result<status::NoContent, Status> {
    connection.run( move |c| match ifus::get(id, c) {
        Ok(_) => ifus::delete(id, c)
            .map(|_| status::NoContent)
            .map_err(|error| error_status(error)),
        Err(error) => Err(error_status(error))
    }).await
}
