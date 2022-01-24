use rocket_sync_db_pools::diesel::{self, prelude::*};
use uuid::Uuid;
use crate::schema::files;
use crate::models::files::{File, InsertableFile};

pub fn all(connection: &diesel::PgConnection) -> QueryResult<Vec<File>> {
    files::table.load::<File>(connection)
}

pub fn get(id: Uuid, connection: &diesel::PgConnection) -> QueryResult<File> {
    files::table.find(id).get_result::<File>(connection)
}

pub fn insert(file: InsertableFile, connection: &diesel::PgConnection) -> QueryResult<File> {
    diesel::insert_into(files::table)
        .values(&file)
        .get_result(connection)
}

pub fn update(id: Uuid, file: File, connection: &diesel::PgConnection) -> QueryResult<File> {
    diesel::update(files::table.find(id))
        .set(&file)
        .get_result(connection)
}

pub fn delete(id: Uuid, connection: &diesel::PgConnection) -> QueryResult<usize> {
    diesel::delete(files::table.find(id))
        .execute(connection)
}
