use rocket::serde::{ Serialize, Deserialize };
use diesel::{ Queryable, Insertable };
use uuid::Uuid;
use crate::schema::files;
use crate::models::Record;

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
pub struct File {
    pub id: Uuid,
    pub name: String,
    pub content_type: String,
    pub size: i64,
    #[serde(skip_serializing)]
    pub data: Vec<u8>,
}

impl Record for File {
    fn id(&self) -> String { self.id.to_string() }
    fn kind(&self) -> String { "file".to_string() }
}

#[derive(Insertable)]
#[table_name = "files"]
pub struct InsertableFile {
    pub name: String,
    pub content_type: String,
    pub size: i64,
    pub data: Vec<u8>,
}
