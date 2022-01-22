use rocket::http::{Status, CookieJar};
use rocket::response::status;
use rocket::serde::json::Json;
use std::str::FromStr;
use crate::connection::DbConn;
use crate::models::products::Product;
use crate::repository::{products, users};
use crate::routes::utils::{ error_status, record_created };

#[get("/products")]
pub async fn all(connection: DbConn) -> Result<Json<Vec<Product>>, Status> {
    connection.run( |c| products::all(c)
        .map(|products| Json(products))
        .map_err(|error| error_status(error))
    ).await
}

#[get("/product/<id>")]
pub async fn get(id: i32, connection: DbConn) -> Result<Json<Product>, Status> {
    connection.run( move |c| products::get(id, c)
        .map(|product| Json(product))
        .map_err(|error| error_status(error))
    ).await
}

#[post("/product", format="application/json", data="<product>")]
pub async fn post(product: Json<Product>, jar: &CookieJar<'_>, connection: DbConn) -> Result<status::Created<Json<Product>>, Status> {
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

    connection.run( |c| products::insert(product.into_inner(), c)
        .map(|product| record_created(product))
        .map_err(|error| error_status(error))
    ).await
}

#[put("/product/<id>", format="application/json", data="<product>")]
pub async fn update(id: i32, product: Json<Product>, jar: &CookieJar<'_>, connection: DbConn) -> Result<Json<Product>, Status> {
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

    connection.run( move |c| products::update(id, product.into_inner(), c)
        .map(|product| Json(product))
        .map_err(|error| error_status(error))
    ).await
}

#[delete("/product/<id>")]
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

    connection.run( move |c| match products::get(id, c) {
        Ok(_) => products::delete(id, c)
            .map(|_| status::NoContent)
            .map_err(|error| error_status(error)),
        Err(error) => Err(error_status(error))
    }).await
}
