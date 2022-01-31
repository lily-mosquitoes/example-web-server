use rocket::serde::{ Serialize, Deserialize };
use rocket::data::ToByteUnit;
use rocket::form::{self, FromFormField, DataField};
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

#[derive(Insertable, Clone, Debug)]
#[table_name = "files"]
pub struct InsertableFile {
    pub name: String,
    pub content_type: String,
    pub size: i64,
    pub data: Vec<u8>,
}

#[rocket::async_trait]
impl<'a> FromFormField<'a> for InsertableFile {
    async fn from_data(field: DataField<'a, '_>) -> form::Result<'a, Self> {
        if field.name.as_name() != "file" {
            return Err(form::Error::validation("field must be named 'file'").into())
        }

        let limit = field.request.limits()
            .get("form-data")
            .unwrap_or(0_usize.bytes());

        let name = field.file_name
            .ok_or(form::error::ErrorKind::Missing)?
            .as_str()
            .ok_or(form::error::ErrorKind::Missing)?
            .to_string();

        if !field.content_type.is_pdf() {
            return Err(form::Error::validation("must be .pdf").into())
        }

        let content_type = field.content_type.to_string();

        let data = match field.data.open(limit).into_bytes().await {
            Ok(bytes) => bytes,
            Err(_) => return Err((None, Some(limit)).into()),
        };

        let data = data.into_inner();
        let data = rocket::request::local_cache!(field.request, data)
            .to_vec();

        let size = data.len() as i64;

        Ok(InsertableFile { name, content_type, size, data, })
    }
}
