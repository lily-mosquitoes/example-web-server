pub mod ifus;
pub mod products;

pub trait Record {
    fn id(&self) -> i32;
}

// use rocket::serde::{ Serialize, Deserialize };
// use diesel::{ Queryable, Insertable };
// use crate::schema::{ ifus, products };
//
// pub trait Record {
//     fn id(&self) -> i32;
// }
//
// #[derive(Queryable, AsChangeset, Serialize, Deserialize)]
// pub struct Ifu {
//     pub id: i32,
//     pub code: String,
//     pub file_url: String,
// }
//
// impl Record for Ifu {
//     fn id(&self) -> i32 { self.id }
// }
//
// #[derive(Insertable)]
// #[table_name = "ifus"]
// pub struct InsertableIfu {
//     pub code: String,
//     pub file_url: String,
// }
//
// impl InsertableIfu {
//     pub fn from_ifu(ifu: Ifu) -> InsertableIfu {
//         InsertableIfu {
//             code: ifu.code,
//             file_url: ifu.file_url,
//         }
//     }
// }
//
// #[derive(Queryable, AsChangeset, Serialize, Deserialize)]
// pub struct Product {
//     pub id: i32,
//     pub code: String,
//     pub name: String,
//     pub ifu_id: i32,
// }
//
// impl Record for Product {
//     fn id(&self) -> i32 { self.id }
// }
//
// #[derive(Insertable)]
// #[table_name = "products"]
// pub struct InsertableProduct {
//     pub code: String,
//     pub name: String,
//     pub ifu_id: i32,
// }
//
// impl InsertableProduct {
//     pub fn from_product(product: Product) -> InsertableProduct {
//         InsertableProduct {
//             code: product.code,
//             name: product.name,
//             ifu_id: product.ifu_id,
//         }
//     }
// }
