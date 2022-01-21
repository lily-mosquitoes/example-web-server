pub mod ifus;

pub mod utils {
    use rocket_sync_db_pools::diesel::result::Error;
    use rocket::http::Status;
    use rocket::response::status;
    use rocket::serde::json::Json;
    use std::env;
    use crate::models;

    pub fn error_status(error: Error) -> Status {
        match error {
            Error::NotFound => Status::NotFound,
            _ => Status::InternalServerError,
        }
    }

    pub fn record_created<T: models::Record>(ifu: T) -> status::Created<Json<T>> {
        let host = env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set");
        let port = env::var("ROCKET_PORT").expect("ROCKET_PORT must be set");
        status::Created::new(
            format!("{host}:{port}/people/{id}", host = host, port = port, id = ifu.id()).to_string()
        ).body(Json(ifu))
    }
}

// #[get("/ifus")]
// pub async fn get_ifus(connection: DbConn) -> Result<Json<Vec<models::Ifu>>, Status> {
//     connection.run( |c| repository::ifus::all(c)
//         .map(|ifus| Json(ifus))
//         .map_err(|error| error_status(error))
//     ).await
// }
//
// #[get("/ifu/<id>")]
// pub async fn get_ifu(id: i32, connection: DbConn) -> Result<Json<models::Ifu>, Status> {
//     connection.run( move |c| repository::ifus::get(id, c)
//         .map(|ifu| Json(ifu))
//         .map_err(|error| error_status(error))
//     ).await
// }
//
// #[post("/ifu", format="application/json", data="<ifu>")]
// pub async fn post_ifu(ifu: Json<models::Ifu>, connection: DbConn) -> Result<status::Created<Json<models::Ifu>>, Status> {
//     connection.run( |c| repository::ifus::insert(ifu.into_inner(), c)
//         .map(|ifu| record_created(ifu))
//         .map_err(|error| error_status(error))
//     ).await
// }
//
// #[put("/ifu/<id>", format="application/json", data="<ifu>")]
// pub async fn update_ifu(id: i32, ifu: Json<models::Ifu>, connection: DbConn) -> Result<Json<models::Ifu>, Status> {
//     connection.run( move |c| repository::ifus::update(id, ifu.into_inner(), c)
//         .map(|ifu| Json(ifu))
//         .map_err(|error| error_status(error))
//     ).await
// }
//
// #[delete("/ifu/<id>")]
// pub async fn delete_ifu(id: i32, connection: DbConn) -> Result<status::NoContent, Status> {
//     connection.run( move |c| match repository::ifus::get(id, c) {
//         Ok(_) => repository::ifus::delete(id, c)
//             .map(|_| status::NoContent)
//             .map_err(|error| error_status(error)),
//         Err(error) => Err(error_status(error))
//     }).await
// }
