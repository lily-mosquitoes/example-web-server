use rocket::serde::{ Serialize, Deserialize };
use diesel::{ Queryable, Insertable };
use crate::schema::{ ifus, products };

#[derive(Queryable, Serialize, Deserialize)]
pub struct Ifu {
    pub id: i32,
    pub code: String,
    pub file_url: String,
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

#[derive(Queryable, Serialize, Deserialize)]
pub struct Product {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub ifu_id: i32,
}

#[derive(Insertable)]
#[table_name = "products"]
pub struct InsertableProduct {
    pub code: String,
    pub name: String,
    pub ifu_id: i32,
}
