use argon2::{password_hash::{self, rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString}, Argon2};
use rocket_sync_db_pools::diesel::{self, prelude::*};
use crate::schema::users;
use crate::models::users::{User, Login, InsertableUser};

fn hash_user_password(password: String) -> Result<String, diesel::result::Error> {
    let salt = SaltString::generate(&mut OsRng);
    match Argon2::default().hash_password(password.as_bytes(), &salt) {
        Ok(hash) => Ok(hash.to_string()),
        Err(_) => Err(diesel::result::Error::RollbackTransaction),
    }
}

fn check_user_password(password: &[u8], hash: &String) -> Result<String, password_hash::errors::Error> {
    let parsed_hash = PasswordHash::new(hash)?;
    if Argon2::default().verify_password(password, &parsed_hash).is_ok() {
        Ok("password verified".to_string())
    } else {
        Err(password_hash::errors::Error::Password)
    }
}

pub fn login(login: Login, connection: &diesel::PgConnection) -> QueryResult<i32> {
    let user_id = users::table.filter(users::username.eq(login.username))
        .select(users::id)
        .first(connection)?;

    let user = users::table.find(user_id).get_result::<User>(connection)?;

    match check_user_password(login.password.as_bytes(), &user.password_hash) {
        Ok(_) => Ok(user_id),
        Err(_) => Err(diesel::result::Error::NotFound),
    }
}

pub fn all(connection: &diesel::PgConnection) -> QueryResult<Vec<User>> {
    users::table.load::<User>(connection)
}

pub fn get(id: i32, connection: &diesel::PgConnection) -> QueryResult<User> {
    users::table.find(id).get_result::<User>(connection)
}

pub fn insert(mut login: Login, connection: &diesel::PgConnection) -> QueryResult<User> {
    login.password = hash_user_password(login.password)?;
    diesel::insert_into(users::table)
        .values(&InsertableUser::from_login(login))
        .get_result(connection)
}

pub fn update(id: i32, login: Login, connection: &diesel::PgConnection) -> QueryResult<User> {
    let mut user = users::table.find(id)
        .get_result::<User>(connection)?;

    match check_user_password(login.password.as_bytes(), &user.password_hash) {
        Ok(_) => {
            user.username = login.username;
        },
        Err(_) => {
            user.username = login.username;
            user.password_hash = hash_user_password(login.password)?;
        },
    };

    diesel::update(users::table.find(id))
        .set(&user)
        .get_result(connection)
}

pub fn delete(id: i32, connection: &diesel::PgConnection) -> QueryResult<usize> {
    diesel::delete(users::table.find(id))
        .execute(connection)
}
