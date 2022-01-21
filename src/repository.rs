// use diesel::{self, prelude::*};
use rocket_sync_db_pools::diesel::{self, prelude::*};
// use crate::connection::DbConn;
use crate::schema::{ifus, products};
use crate::models::{Ifu, InsertableIfu};

pub fn all_ifus(connection: &diesel::PgConnection) -> QueryResult<Vec<Ifu>> {
    ifus::table.load::<Ifu>(connection)
}

pub fn insert_ifu(ifu: Ifu, connection: &diesel::PgConnection) -> QueryResult<Ifu> {
    diesel::insert_into(ifus::table)
        .values(&InsertableIfu::from_ifu(ifu))
        .get_result(connection)
}
