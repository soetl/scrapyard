use crate::{models::user::User, schema::users};

use diesel::{
    prelude::*,
    result::{DatabaseErrorKind, Error},
};
use scrypt::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Scrypt,
};
use serde::Deserialize;

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password_hash: &'a str,
}

pub enum UserCreationError {
    UsernameTaken,
    UnknownViolation(String),
    Unknown(Error),
}

impl From<Error> for UserCreationError {
    fn from(e: Error) -> Self {
        match e {
            Error::DatabaseError(DatabaseErrorKind::UniqueViolation, e) => {
                match e.constraint_name() {
                    Some("users_username_key") => UserCreationError::UsernameTaken,
                    _ => UserCreationError::UnknownViolation(
                        e.constraint_name().unwrap_or("Unknown").to_owned(),
                    ),
                }
            }
            e => UserCreationError::Unknown(e),
        }
    }
}

pub fn create(
    conn: &mut PgConnection,
    username: &str,
    password: &str,
) -> Result<User, UserCreationError> {
    let salt = SaltString::generate(&mut OsRng);

    let hash = Scrypt
        .hash_password(password.as_bytes(), &salt)
        .expect("Failed to hash password")
        .to_string()
        ;

    let new_user = NewUser {
        username,
        password_hash: &hash[..],
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result::<User>(conn)
        .map_err(|e| e.into())
}

pub fn login(conn: &mut PgConnection, username: &str, password: &str) -> Option<User> {
    let user = users::table
        .filter(users::username.eq(username))
        .get_result::<User>(conn)
        .map_err(|e| eprintln!("login_user: {}", e))
        .ok()?;

    let parsed_hash = PasswordHash::new(&user.password_hash).unwrap();
    let password_match = Scrypt
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|e| eprintln!("login_user: scrypt check: {}", e))
        .is_ok();

    match password_match {
        true => Some(user),
        false => {
            eprintln!("login_user: password mismatch for user {}", username);
            None
        }
    }
}

pub fn find(conn: &mut PgConnection, id: i32) -> Option<User> {
    users::table
        .find(id)
        .get_result::<User>(conn)
        .map_err(|e| eprintln!("find_user: {}", e))
        .ok()
}

pub fn find_by_username(conn: &mut PgConnection, username: &str) -> Option<User> {
    users::table
        .filter(users::username.eq(username))
        .get_result::<User>(conn)
        .map_err(|e| eprintln!("find_user: {}", e))
        .ok()
}

#[derive(Deserialize, AsChangeset, Default, Clone)]
#[diesel(table_name = users)]
pub struct UpdateUserData {
    username: Option<String>,
    image: Option<String>,

    #[diesel(column_name = password_hash)]
    password: Option<String>,
}

pub fn update(conn: &mut PgConnection, id: i32, data: &UpdateUserData) -> Option<User> {
    let data = &UpdateUserData {
        username: None,
        password: None,
        ..data.clone()
    };

    diesel::update(users::table.find(id))
        .set(data)
        .get_result(conn)
        .ok()
}
