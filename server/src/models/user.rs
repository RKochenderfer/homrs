use crate::cust_error::{Result, Error};
use crate::schema::{users};
use chrono::NaiveDateTime;
use diesel::{RunQueryDsl, insert_into, PgConnection};
use argon2::{self, Config};
use serde::{Deserialize, Serialize};
// use crate::db::PgPooledConnection;

/// Struct modeling a user in the database
#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    #[serde(skip)]
    pub password_hash: String,
    pub user_role: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl User {
    /// Inserts a new user into the database
    pub fn create_user(conn: &PgConnection, new_user: &PostUser) -> Result<User> {
        use crate::schema::users::dsl::*;

        let salt = std::env::var("SALT")?;
        let config = Config::default();
        let hash = argon2::hash_encoded(new_user.password.as_bytes(), salt.as_bytes(), &config)?;
        let created = chrono::Utc::now().naive_utc();
        let updated = created.clone();
        let insert = InsertUser {
            email: new_user.email.to_owned(),
            first_name: new_user.first_name.to_owned(),
            last_name: new_user.last_name.to_owned(),
            password_hash: hash,
            user_role: new_user.user_role.to_owned(),
            created_at: created,
            updated_at: updated,
        };

        Ok(insert_into(users).values(&insert).get_result(conn).expect("Failed to insert user."))
    }
}

/// The struct used to insert a user into the database
#[derive(Insertable)]
#[table_name = "users"]
pub struct InsertUser {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password_hash: String,
    pub user_role: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct PostUser<'a> {
    pub email: &'a str,
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub password: &'a str,
    pub user_role: &'a str,
}

impl<'a> PostUser<'a> {
    /// Enforces the user uses a strong password.
    ///
    /// Password requirements:
    ///     * Minimum Length: 8
    ///     * Maximum Length: 20
    ///     * Has at least one lower case letter
    ///     * Has at least one upper case letter
    ///     * Has at least one number
    ///     * Has at least one special character
    pub fn check_password_strength(&self) -> Result<()>{
        let mut lowercase = false;
        let mut uppercase = false;
        let mut digit = false;
        let mut symbol = false;
        let mut valid = false;
        let specials: &str = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";

        if self.password.len() < 8 || self.password.len() > 20 {
            return Err(Error::boxed("Password must between 8 and 20 characters long."));
        }

        for c in self.password.chars() {
            if c.is_lowercase() {
                lowercase = true
            } else if c.is_uppercase() {
                uppercase = true
            } else if c.is_numeric() {
                digit = true
            } else if specials.contains(c) {
                symbol = true
            }
            if lowercase && uppercase && digit && symbol {
                valid = true;
                break;
            }
        }

        if valid {
            Ok(())
        } else {
            Err(Error::boxed("Password must have at least one lower case letter, \
            one upper case letter, one number, and one special character"))
        }
    }
}
