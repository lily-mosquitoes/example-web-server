use rocket::serde::{ Serialize, Deserialize };
use diesel::{ Queryable, Insertable };
use chrono::{ Utc, DateTime };
use crate::schema::users;
use crate::models::Record;

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub last_login: DateTime<Utc>,
    pub admin_status: bool,
}

impl Record for User {
    fn id(&self) -> i32 { self.id }
}

#[derive(Deserialize)]
pub struct Login {
    pub username: String,
    pub password: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct InsertableUser {
    pub username: String,
    pub password_hash: String,
}

impl InsertableUser {
    pub fn from_login(login: Login) -> InsertableUser {
        InsertableUser {
            username: login.username,
            password_hash: login.password,
        }
    }
}
