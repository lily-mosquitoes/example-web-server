use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use crate::connection::DbConn;
use crate::models::users::User;
use crate::repository::users;
use crate::routes::utils::{ error_status, record_created };

#[get("/users")]
pub async fn all(connection: DbConn) -> Result<Json<Vec<User>>, Status> {
    connection.run( |c| users::all(c)
        .map(|users| Json(users))
        .map_err(|error| error_status(error))
    ).await
}

#[get("/user/<id>")]
pub async fn get(id: i32, connection: DbConn) -> Result<Json<User>, Status> {
    connection.run( move |c| users::get(id, c)
        .map(|user| Json(user))
        .map_err(|error| error_status(error))
    ).await
}

#[post("/user", format="application/json", data="<user>")]
pub async fn post(user: Json<User>, connection: DbConn) -> Result<status::Created<Json<User>>, Status> {
    connection.run( |c| users::insert(user.into_inner(), c)
        .map(|user| record_created(user))
        .map_err(|error| error_status(error))
    ).await
}

#[put("/user/<id>", format="application/json", data="<user>")]
pub async fn update(id: i32, user: Json<User>, connection: DbConn) -> Result<Json<User>, Status> {
    connection.run( move |c| users::update(id, user.into_inner(), c)
        .map(|user| Json(user))
        .map_err(|error| error_status(error))
    ).await
}

#[delete("/user/<id>")]
pub async fn delete(id: i32, connection: DbConn) -> Result<status::NoContent, Status> {
    connection.run( move |c| match users::get(id, c) {
        Ok(_) => users::delete(id, c)
            .map(|_| status::NoContent)
            .map_err(|error| error_status(error)),
        Err(error) => Err(error_status(error))
    }).await
}
