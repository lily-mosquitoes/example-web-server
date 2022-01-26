use rocket_dyn_templates::Template;
use std::collections::HashMap;
use rocket::form::Form;

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
pub fn search(search: Form<Search>) -> Template {
    println!("{:?}", search);
    let mut context = HashMap::<String, String>::new();
    context.insert("search".to_string(), search.into_inner().code);
    Template::render("index", &context)
}
