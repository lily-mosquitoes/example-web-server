use rocket_dyn_templates::Template;
use std::collections::HashMap;
use rocket::form::Form;
use rocket::http::Status;
use crate::connection::DbConn;
use crate::models::ifus::Ifu;
use crate::repository::ifus;
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
    let code = search.into_inner().code;

    let result = connection.run( move |c| ifus::search(code, c)).await;

    match result {
        Ok(id) => {
            context.insert("ifu_id".to_string(), id);
            (Status::Ok, Template::render("index", &context))
        },
        Err(error) => {
            context.insert("not_found".to_string(), 404);
            (Status::NotFound, Template::render("index", &context))
        },
    }

    // Template::render("index", &context)
}
