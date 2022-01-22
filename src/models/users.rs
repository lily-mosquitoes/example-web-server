use rocket::serde::{ Serialize, Deserialize };
use diesel::{ Queryable, Insertable };
use chrono::{ Utc, DateTime };
use crate::schema::users;
use crate::models::Record;

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub last_login: DateTime<Utc>,
}

impl Record for User {
    fn id(&self) -> i32 { self.id }
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct InsertableUser {
    pub username: String,
    pub password_hash: String,
}

impl InsertableUser {
    pub fn from_user(user: User) -> InsertableUser {
        InsertableUser {
            username: user.username,
            password_hash: user.password_hash,
        }
    }
}
