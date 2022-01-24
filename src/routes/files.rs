use rocket::{Data, response::status, http::{Status, ContentType, CookieJar}};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use rocket::serde::json::Json;
use std::str::FromStr;
use rocket_multipart_form_data::{mime, MultipartFormDataOptions, MultipartFormData, MultipartFormDataError, MultipartFormDataField};
use crate::connection::DbConn;
use crate::models::files::{File, InsertableFile};
use crate::repository::{files, users};
use crate::routes::utils::{ error_status, record_created };

#[get("/files")]
pub async fn all(connection: DbConn) -> Result<Json<Vec<File>>, Status> {
    connection.run( |c| files::all(c)
        .map(|files| Json(files))
        .map_err(|error| error_status(error))
    ).await
}

#[post("/file", format="multipart/form-data", data = "<data>")]
pub async fn post(content_type: &ContentType, data: Data<'_>, jar: &CookieJar<'_>, connection: DbConn) -> Result<status::Created<Json<File>>, Status> {
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

    let options = MultipartFormDataOptions::with_multipart_form_data_fields(
        vec! [
            match MultipartFormDataField::file("file").content_type_by_string(Some(mime::APPLICATION_PDF)) {
                Ok(form) => form,
                Err(_) => return Err(Status::BadRequest),
            },
        ]
    );

    let multipart_form_data = match MultipartFormData::parse(content_type, data, options).await {
        Ok(form_data) => form_data,
        Err(error) => match error {
            MultipartFormDataError::DataTooLargeError(_) => return Err(Status::PayloadTooLarge),
            _ => return Err(Status::BadRequest),
        },
    };

    match multipart_form_data.files.get("file") {
        Some(file_fields) => {
            let file_field = &file_fields[0];
            let file_name = match &file_field.file_name {
                Some(string) => string.to_string(),
                None => return Err(Status::BadRequest),
            };
            let content_type = match &file_field.content_type {
                Some(string) => string.to_string(),
                None => return Err(Status::BadRequest),
            };
            let size = match std::fs::metadata(&file_field.path) {
                Ok(metadata) => metadata.len() as i64,
                Err(_) => return Err(Status::InternalServerError),
            };
            let data = match std::fs::read(&file_field.path) {
                Ok(vec) => vec,
                Err(_) => return Err(Status::InternalServerError),
            };

            let insert_file = InsertableFile {
                name: file_name,
                content_type: content_type,
                size: size,
                data: data,
            };

            connection.run( |c| files::insert(insert_file, c)
                .map(|file| record_created(file))
                .map_err(|error| error_status(error))
            ).await
        },
        None => Err(Status::BadRequest),
    }
}


// #[post("/file", data = "<data>")]
// pub async fn post(content_type: &ContentType, data: Data<'_>) -> Result<std::fs::File, Status> {
//     let mut options = MultipartFormDataOptions::with_multipart_form_data_fields(
//         vec! [
//             match MultipartFormDataField::file("file").content_type_by_string(Some(mime::APPLICATION_PDF)) {
//                 Ok(form) => form,
//                 Err(_) => return Err(Status::BadRequest),
//             },
//         ]
//     );
//
//     let mut multipart_form_data = match MultipartFormData::parse(content_type, data, options).await {
//         Ok(form_data) => form_data,
//         Err(error) => match error {
//             MultipartFormDataError::DataTooLargeError(_) => return Err(Status::PayloadTooLarge),
//             _ => return Err(Status::BadRequest),
//         },
//     };
//
//     let file = multipart_form_data.files.get("file");
//
//     if let Some(file_fields) = file {
//         let file_field = &file_fields[0];
//         let file_name = match &file_field.file_name {
//             Some(string) => string.to_string(),
//             None => return Err(Status::BadRequest),
//         };
//         let content_type = match &file_field.content_type {
//             Some(string) => string.to_string(),
//             None => return Err(Status::BadRequest),
//         };
//         let size = match std::fs::metadata(&file_field.path) {
//             Ok(metadata) => metadata.len() as i64,
//             Err(_) => return Err(Status::BadRequest),
//         };
//         let data = match std::fs::read(&file_field.path) {
//             Ok(vec) => vec,
//             Err(_) => return Err(Status::BadRequest),
//         };
//         let mut temp_path = "/tmp/rs-resp-".to_string();
//         let temp_file_name: String = thread_rng()
//             .sample_iter(&Alphanumeric)
//             .take(30)
//             .map(char::from)
//             .collect();
//         temp_path.push_str(temp_file_name.as_str());
//         let write_path = std::path::Path::new(temp_path.as_str());
//         let resp = match std::fs::write(&write_path, &data) {
//             Ok(_) => match std::fs::File::open(&write_path) {
//                 Ok(file) => {
//                     std::fs::remove_file(&write_path).unwrap();
//                     return Ok(file)
//                 },
//                 Err(_) => return Err(Status::InternalServerError),
//             },
//             Err(_) => return Err(Status::InternalServerError),
//         };
//         // return resp;
//
//         // let _content_type = &file_field.content_type;
//         // let _file_name = &file_field.file_name;
//         // let _path = &file_field.path;
//         // let len = std::fs::metadata(_path).unwrap().len();
//         // let data = std::fs::read(_path).unwrap();
//         // let insert_file = InsertableFile {
//         //     name: format!("{}", _file_name.unwrap().as_ref()),
//         //     content_type: format!("{}", _content_type.unwrap().as_ref()),
//         //     size: len as i64,
//         //     data: data,
//         // };
//     }
//     Err(Status::BadRequest)
//     // File::open("hjdjkahj.txt").ok()
// }