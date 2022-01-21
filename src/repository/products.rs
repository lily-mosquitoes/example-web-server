use rocket_sync_db_pools::diesel::{self, prelude::*};
use crate::schema::products;
use crate::models::products::{Product, InsertableProduct};

pub fn all(connection: &diesel::PgConnection) -> QueryResult<Vec<Product>> {
    products::table.load::<Product>(connection)
}

pub fn get(id: i32, connection: &diesel::PgConnection) -> QueryResult<Product> {
    products::table.find(id).get_result::<Product>(connection)
}

pub fn insert(product: Product, connection: &diesel::PgConnection) -> QueryResult<Product> {
    diesel::insert_into(products::table)
        .values(&InsertableProduct::from_product(product))
        .get_result(connection)
}

pub fn update(id: i32, product: Product, connection: &diesel::PgConnection) -> QueryResult<Product> {
    diesel::update(products::table.find(id))
        .set(&product)
        .get_result(connection)
}

pub fn delete(id: i32, connection: &diesel::PgConnection) -> QueryResult<usize> {
    diesel::delete(products::table.find(id))
        .execute(connection)
}
