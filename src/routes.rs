pub mod ifus;
pub mod products;
pub mod users;
pub mod files;
pub mod healthcheck;
pub mod website;

pub mod utils {
    use rocket_sync_db_pools::diesel::result::Error;
    use rocket::http::Status;
    use rocket::response::status;
    use rocket::serde::json::Json;
    use std::env;
    use rand::{thread_rng, Rng};
    use rand::distributions::Alphanumeric;
    use crate::models;

    pub fn new_temp_path() -> std::path::PathBuf {
        let mut temp_path = "/tmp/rs-".to_string();
        let temp_file_name: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect();
        temp_path.push_str(temp_file_name.as_str());

        std::path::PathBuf::from(temp_path.as_str())
    }

    pub fn error_status(error: Error) -> Status {
        match error {
            Error::NotFound => Status::NotFound,
            _ => Status::InternalServerError,
        }
    }

    pub fn record_created<T: models::Record>(record: T) -> status::Created<Json<T>> {
        let host = env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set");
        let port = env::var("ROCKET_PORT").expect("ROCKET_PORT must be set");
        status::Created::new(
            format!("{host}:{port}/{kind}/{id}", host = host, port = port, kind = record.kind(), id = record.id())
        ).body(Json(record))
    }
}
