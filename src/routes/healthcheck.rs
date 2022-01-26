use rocket::response::status;
use rocket::http::Status;
use chrono::Utc;

#[get("/healthcheck")]
pub fn ok() -> status::Custom<String> {
    let datetime = Utc::now();
    status::Custom(Status::Ok, format!("healthcheck completed - {}", datetime))
}
