use rocket_dyn_templates::Template;
use std::collections::HashMap;

#[get("/")]
pub fn index() -> Template {
    let context = HashMap::<String, String>::new();
    Template::render("index", &context)
}