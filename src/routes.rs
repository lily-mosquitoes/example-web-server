#[get("/")]
pub fn index() -> &'static str {
    "Works!"
}
