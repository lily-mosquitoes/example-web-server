use rocket::http::{Status, Cookie, CookieJar};
use rocket::response::status;
use rocket::serde::json::Json;
use std::str::FromStr;
use diesel::result::Error;
use crate::connection::DbConn;
use crate::models::users::{User, Login};
use crate::repository::users;
use crate::routes::utils::{ error_status, record_created };

#[post("/login", data="<login>")]
pub async fn login(jar: &CookieJar<'_>, login: Json<Login>, connection: DbConn) -> Result<status::Accepted<String>, status::Forbidden<String>> {
    let result: Result<i32, Error> = connection.run( move |c| users::login(login.into_inner(), c)
    ).await;
    match result {
        Ok(id) => {
            jar.add_private(Cookie::new("user_id", id.to_string()));
            Ok(status::Accepted(Some("User logged in".to_string())))
        },
        Err(_) => Err(status::Forbidden(Some("Invalid username/password".to_string()))),
    }
}

#[get("/user_id")]
pub fn user_id(jar: &CookieJar<'_>) -> Option<String> {
    jar.get_private("user_id")
        .map(|crumb| format!("User ID: {}", crumb.value()))
}

#[post("/logout")]
pub fn logout(jar: &CookieJar<'_>) -> Json<String> {
    jar.remove_private(Cookie::named("user_id"));
    Json("User logged out".to_string())
}

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

#[post("/user", format="application/json", data="<login>")]
pub async fn post(login: Json<Login>, jar: &CookieJar<'_>, connection: DbConn) -> Result<status::Created<Json<User>>, Status> {
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

    connection.run( |c| users::insert(login.into_inner(), c)
        .map(|user| record_created(user))
        .map_err(|error| error_status(error))
    ).await
}

#[put("/user/<id>", format="application/json", data="<login>")]
pub async fn update(id: i32, login: Json<Login>, jar: &CookieJar<'_>, connection: DbConn) -> Result<Json<User>, Status> {
    match jar.get_private("user_id") {
        Some(crumb) => {
            let user_id = match FromStr::from_str(crumb.value()) {
                Ok(user_id) => user_id,
                Err(_) => return Err(Status::Forbidden),
            };
            match connection.run( move |c| users::get(user_id, c)
            ).await {
                Ok(_) => {
                    if user_id != id {
                        return Err(Status::Forbidden)
                    }
                },
                Err(_) => return Err(Status::Forbidden),
            };
        },
        None => return Err(Status::Forbidden),
    };

    connection.run( move |c| users::update(id, login.into_inner(), c)
        .map(|user| Json(user))
        .map_err(|error| error_status(error))
    ).await
}

#[delete("/user/<id>")]
pub async fn delete(id: i32, jar: &CookieJar<'_>, connection: DbConn) -> Result<status::NoContent, Status> {
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

    match id {
        1 => return Err(Status::Forbidden), // avoid deletion of default user
        _ => (),
    };

    connection.run( move |c| match users::get(id, c) {
        Ok(_) => users::delete(id, c)
            .map(|_| status::NoContent)
            .map_err(|error| error_status(error)),
        Err(error) => Err(error_status(error))
    }).await
}
