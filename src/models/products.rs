use rocket::serde::{ Serialize, Deserialize };
use diesel::{ Queryable, Insertable };
use crate::schema::products;
use crate::models::Record;

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
pub struct Product {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub ifu_id: i32,
}

impl Record for Product {
    fn id(&self) -> String { self.id.to_string() }
    fn kind(&self) -> String { "product".to_string() }
}

#[derive(Insertable)]
#[table_name = "products"]
pub struct InsertableProduct {
    pub code: String,
    pub name: String,
    pub ifu_id: i32,
}

impl InsertableProduct {
    pub fn from_product(product: Product) -> InsertableProduct {
        InsertableProduct {
            code: product.code,
            name: product.name,
            ifu_id: product.ifu_id,
        }
    }
}
