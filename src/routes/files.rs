use rocket::{response::status, http::{Status, CookieJar}};
use rocket::serde::json::Json;
use std::str::FromStr;
use uuid::Uuid;
use rocket::form::{Form, Contextual};
use crate::connection::DbConn;
use crate::models::files::{File, InsertableFile};
use crate::repository::{files, users};
use crate::routes::utils::{ new_temp_path, error_status, record_created };

#[get("/files")]
pub async fn all(connection: DbConn) -> Result<Json<Vec<File>>, Status> {
    connection.run( |c| files::all(c)
        .map(|files| Json(files))
        .map_err(|error| error_status(error))
    ).await
}

#[get("/file/<id>")]
pub async fn get(id: Uuid, connection: DbConn) -> Result<Json<File>, Status> {
    connection.run( move |c| files::get(id, c)
        .map(|file| Json(file))
        .map_err(|error| error_status(error))
    ).await
}

#[get("/file/download/<id>")]
pub async fn download(id: Uuid, connection: DbConn) -> Result<std::fs::File, Status> {
    connection.run( move |c| match files::get(id, c) {
        Ok(file) => {
            let write_path = new_temp_path();
            match std::fs::write(&write_path, &file.data) {
                Ok(_) => match std::fs::File::open(&write_path) {
                    Ok(file) => {
                        std::fs::remove_file(&write_path).unwrap();
                        return Ok(file)
                    },
                    Err(_) => return Err(Status::InternalServerError),
                },
                Err(_) => return Err(Status::InternalServerError),
            };
        },
        Err(error) => Err(error_status(error)),
    }).await
}

#[post("/file", format="multipart/form-data", data = "<form>")]
pub async fn post(form: Form<Contextual<'_, InsertableFile>>, connection: DbConn) -> Result<status::Created<Json<File>>, (Status, String)> {
    if let Some(insertable_file) = form.value.clone() {
        return match connection.run( |c| files::insert(insertable_file, c)
            // .map(|file| record_created(file))
            // .map_err(|error| error_status(error))
        ).await {
            Ok(file) => Ok(record_created(file)),
            Err(error) => Err((error_status(error), "failed to create record".to_string()))
        }
    }

    if let Some(error) = form.context.errors().last() {
        match error.status().code {
            413 => Err((Status::PayloadTooLarge, format!("payload too large, {}", error.kind))),
            422 => Err((Status::UnprocessableEntity, format!("file {}", error.kind))),
            _ => Err((Status::BadRequest, format!("{}", error.kind))),
        }
    } else {
        // form must have errors if value failed to read
        Err((Status::InternalServerError, "failed to process request properly".to_string()))
    }
}

#[delete("/file/<id>")]
pub async fn delete(id: Uuid, jar: &CookieJar<'_>, connection: DbConn) -> Result<status::NoContent, Status> {
    match jar.get_private("user_id") {
        Some(crumb) => {
            let id = match FromStr::from_str(crumb.value()) {
                Ok(id) => id,
                Err(_) => return Err(Status::Forbidden),
            };
            match connection.run( move |c| users::get(id, c)
                .map(|user| Json(user))
                .map_err(|error| error_status(error))
            ).await {
                Ok(_) => (),
                Err(_) => return Err(Status::Forbidden),
            };
        },
        None => return Err(Status::Forbidden),
    };

    connection.run( move |c| match files::get(id, c) {
        Ok(_) => files::delete(id, c)
            .map(|_| status::NoContent)
            .map_err(|error| error_status(error)),
        Err(error) => Err(error_status(error))
    }).await
}
