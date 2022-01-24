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
    fn id(&self) -> String { self.id.to_string() }
    fn kind(&self) -> String { "ifus".to_string() }
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
