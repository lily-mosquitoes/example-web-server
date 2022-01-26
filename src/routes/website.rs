use rocket_dyn_templates::Template;
use std::collections::HashMap;
use rocket::form::Form;
use rocket::http::Status;
use crate::connection::DbConn;
use crate::repository::{ifus, products};
use crate::routes::utils::error_status;

#[derive(Debug, FromForm)]
pub struct Search {
    code: String
}

#[get("/")]
pub fn index() -> Template {
    let context = HashMap::<String, String>::new();
    Template::render("index", &context)
}

#[post("/", format="multipart/form-data", data="<search>")]
pub async fn search(search: Form<Search>, connection: DbConn) -> (Status, Template) {
    let mut context = HashMap::<String, i32>::new();
    let search = search.into_inner();
    let ifu_code = search.code.clone();
    let product_code = search.code.clone();

    let ifu = connection.run( |c| ifus::search(ifu_code, c)).await;
    let product = connection.run( |c| products::search(product_code, c)).await;

    match (ifu, product) {
        (Ok(id), Err(_)) => {
            context.insert("id".to_string(), id);
            (Status::Ok, Template::render("index", &context))
        },
        (Err(_), Ok(id)) => {
            context.insert("id".to_string(), id);
            (Status::Ok, Template::render("index", &context))
        },
        _ => {
            context.insert("not_found".to_string(), 404);
            (Status::NotFound, Template::render("index", &context))
        },
    }
}
