use rocket::serde::{ Serialize, Deserialize };
use diesel::{ Queryable, Insertable };
use crate::schema::ifus;
use crate::models::Record;

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
pub struct Ifu {
    pub id: i32,
    pub code: String,
    pub file_url: String,
}

impl Record for Ifu {
    fn id(&self) -> i32 { self.id }
}

#[derive(Insertable)]
#[table_name = "ifus"]
pub struct InsertableIfu {
    pub code: String,
    pub file_url: String,
}

impl InsertableIfu {
    pub fn from_ifu(ifu: Ifu) -> InsertableIfu {
        InsertableIfu {
            code: ifu.code,
            file_url: ifu.file_url,
        }
    }
}
