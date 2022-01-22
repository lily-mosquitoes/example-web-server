use argon2::{password_hash::{self, rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString}, Argon2};
use rocket_sync_db_pools::diesel::{self, prelude::*};
use crate::schema::users;
use crate::models::users::{User, InsertableUser};

fn hash_user_password(password: String) -> Result<String, diesel::result::Error> {
    let salt = SaltString::generate(&mut OsRng);
    match Argon2::default().hash_password(password.as_bytes(), &salt) {
        Ok(hash) => Ok(hash.to_string()),
        Err(_) => Err(diesel::result::Error::RollbackTransaction),
    }
}

// #[derive(Debug, Clone)]
// struct NoMatch;
// type Result<T> = std::result::Result<T, NoMatch>;

fn check_user_password(password: &[u8], hash: &String) -> Result<String, password_hash::errors::Error> {
    let parsed_hash = PasswordHash::new(hash)?;
    if Argon2::default().verify_password(password, &parsed_hash).is_ok() {
        Ok("password verified".to_string())
    } else {
        Err(password_hash::errors::Error::Password)
    }
}

pub fn all(connection: &diesel::PgConnection) -> QueryResult<Vec<User>> {
    users::table.load::<User>(connection)
}

pub fn get(id: i32, connection: &diesel::PgConnection) -> QueryResult<User> {
    users::table.find(id).get_result::<User>(connection)
}

pub fn insert(mut user: User, connection: &diesel::PgConnection) -> QueryResult<User> {
    user.password_hash = hash_user_password(user.password_hash)?;
    diesel::insert_into(users::table)
        .values(&InsertableUser::from_user(user))
        .get_result(connection)
}

pub fn update(id: i32, mut user: User, connection: &diesel::PgConnection) -> QueryResult<User> {
    let old_user = users::table.find(id)
        .get_result::<User>(connection)?;

    match check_user_password(user.password_hash.as_bytes(), &old_user.password_hash) {
        Ok(_) => user.password_hash = old_user.password_hash,
        Err(_) => user.password_hash = hash_user_password(user.password_hash)?,
    };

    diesel::update(users::table.find(id))
        .set(&user)
        .get_result(connection)
}

pub fn delete(id: i32, connection: &diesel::PgConnection) -> QueryResult<usize> {
    diesel::delete(users::table.find(id))
        .execute(connection)
}
