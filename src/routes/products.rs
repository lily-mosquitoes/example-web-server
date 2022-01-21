use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use crate::connection::DbConn;
use crate::models::products::Product;
use crate::repository::products;
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
pub async fn post(product: Json<Product>, connection: DbConn) -> Result<status::Created<Json<Product>>, Status> {
    connection.run( |c| products::insert(product.into_inner(), c)
        .map(|product| record_created(product))
        .map_err(|error| error_status(error))
    ).await
}

#[put("/product/<id>", format="application/json", data="<product>")]
pub async fn update(id: i32, product: Json<Product>, connection: DbConn) -> Result<Json<Product>, Status> {
    connection.run( move |c| products::update(id, product.into_inner(), c)
        .map(|product| Json(product))
        .map_err(|error| error_status(error))
    ).await
}

#[delete("/product/<id>")]
pub async fn delete(id: i32, connection: DbConn) -> Result<status::NoContent, Status> {
    connection.run( move |c| match products::get(id, c) {
        Ok(_) => products::delete(id, c)
            .map(|_| status::NoContent)
            .map_err(|error| error_status(error)),
        Err(error) => Err(error_status(error))
    }).await
}
