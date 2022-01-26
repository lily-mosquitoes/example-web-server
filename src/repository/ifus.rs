use rocket_sync_db_pools::diesel::{self, prelude::*};
use crate::schema::ifus;
use crate::models::ifus::{Ifu, InsertableIfu};

pub fn all(connection: &diesel::PgConnection) -> QueryResult<Vec<Ifu>> {
    ifus::table.load::<Ifu>(connection)
}

pub fn get(id: i32, connection: &diesel::PgConnection) -> QueryResult<Ifu> {
    ifus::table.find(id)
        .get_result::<Ifu>(connection)
}

pub fn search(code: String, connection: &diesel::PgConnection) -> QueryResult<i32> {
    ifus::table.filter(ifus::code.eq(code))
        .select(ifus::id)
        .first(connection)
}

pub fn insert(ifu: Ifu, connection: &diesel::PgConnection) -> QueryResult<Ifu> {
    diesel::insert_into(ifus::table)
        .values(&InsertableIfu::from_ifu(ifu))
        .get_result(connection)
}

pub fn update(id: i32, ifu: Ifu, connection: &diesel::PgConnection) -> QueryResult<Ifu> {
    diesel::update(ifus::table.find(id))
        .set(&ifu)
        .get_result(connection)
}

pub fn delete(id: i32, connection: &diesel::PgConnection) -> QueryResult<usize> {
    diesel::delete(ifus::table.find(id))
        .execute(connection)
}
