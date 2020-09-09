use crate::cust_error::{Error, Result};
use crate::models::session::Session;
use crate::schema::users;
use crate::schema::users::dsl::*;
use argon2::{self, Config};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::query_dsl::SaveChangesDsl;
use diesel::{insert_into, PgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};

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
    pub fn create(conn: &PgConnection, new_user: &PostUser) -> Result<User> {
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

        Ok(insert_into(users)
            .values(&insert)
            .get_result(conn)
            .expect("Failed to insert user."))
    }

    /// Gets a user with the given user id
    pub fn get_by_id(conn: &PgConnection, user_id: i32) -> Result<User> {
        let result = users
            .find(user_id)
            .first(conn)
            .expect("Error finding user.");

        Ok(result)
    }

    /// Gets a user with a given email
    pub fn get_by_email(conn: &PgConnection, user_email: &str) -> Result<Option<User>> {
        let mut result: Vec<User> = users
            .filter(email.eq(user_email))
            .limit(1)
            .load::<User>(conn)
            .expect("Failed to load user.");

        if result.len() > 0 {
            Ok(Some(result.pop().unwrap()))
        } else {
            // Err(Error::boxed(&format!("Could not find user with email: {}", user_email)))
            Ok(None)
        }
    }

    /// delete user
    pub fn delete(conn: &PgConnection, user_id: i32, session_token: String) -> Result<()> {
        // Delete from sessions table
        let session = Session::get_by_token(conn, &session_token)?;
        Session::delete(conn, session.unwrap().id)?;
        // Delete user from table
        let deleted_count = diesel::delete(users.filter(id.eq(user_id))).execute(conn)?;

        if deleted_count > 0 {
            Ok(())
        } else {
            Err(Error::boxed(&format!("User id {} not found", user_id)))
        }
    }

    /// Updates a user's information except for password
    pub fn update(conn: &PgConnection, put_user: PutUser, user_id: i32) -> Result<User> {
        let user = User::get_by_id(conn, user_id)?;
        let mut new_email = user.email;
        let mut new_first_name = user.first_name;
        let mut new_last_name = user.last_name;
        let mut new_user_role = user.user_role;

        if let Some(x) = put_user.email {
            new_email = x.to_owned()
        }
        if let Some(x) = put_user.first_name {
            new_first_name = x.to_owned()
        }
        if let Some(x) = put_user.last_name {
            new_last_name = x.to_owned()
        }
        if let Some(x) = put_user.user_role {
            new_user_role = x.to_owned()
        }

        let updated = UpdateUser::new(
            user.id,
            new_email,
            new_first_name,
            new_last_name,
            new_user_role,
        );

        Ok(updated.save_changes(conn)?)
    }

    pub fn update_password(conn: &PgConnection, password: &str, user_id: i32) -> Result<()> {
        let salt = std::env::var("SALT")?;
        let config = Config::default();
        let hash = argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config)?;

        let updated = UpdateUserPassword::new(user_id, &hash);

        let _ = updated
            .save_changes::<User>(conn)
            .expect("Failed to update password.");

        Ok(())
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
    pub fn check_password_strength(&self) -> Result<()> {
        let mut lowercase = false;
        let mut uppercase = false;
        let mut digit = false;
        let mut symbol = false;
        let mut valid = false;
        let specials: &str = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";

        if self.password.len() < 8 || self.password.len() > 20 {
            return Err(Error::boxed(
                "Password must between 8 and 20 characters long.",
            ));
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
            Err(Error::boxed(
                "Password must have at least one lower case letter, \
            one upper case letter, one number, and one special character",
            ))
        }
    }
}

#[derive(AsChangeset, Identifiable)]
#[table_name = "users"]
pub struct UpdateUser {
    pub id: i32,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub user_role: String,
    pub updated_at: NaiveDateTime,
}

impl UpdateUser {
    pub fn new(
        user_id: i32,
        new_email: String,
        new_first_name: String,
        new_last_name: String,
        new_user_role: String,
    ) -> UpdateUser {
        UpdateUser {
            id: user_id,
            email: new_email,
            first_name: new_first_name,
            last_name: new_last_name,
            user_role: new_user_role,
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}

#[derive(Deserialize)]
pub struct PutUser<'a> {
    pub email: Option<&'a str>,
    pub first_name: Option<&'a str>,
    pub last_name: Option<&'a str>,
    pub user_role: Option<&'a str>,
}

#[derive(AsChangeset, Identifiable)]
#[table_name = "users"]
pub struct UpdateUserPassword {
    pub id: i32,
    pub password_hash: String,
    pub updated_at: NaiveDateTime,
}

impl UpdateUserPassword {
    pub fn new(user_id: i32, new_hash: &str) -> UpdateUserPassword {
        UpdateUserPassword {
            id: user_id,
            password_hash: new_hash.to_owned(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}

#[derive(Deserialize)]
pub struct ChangePassword<'a> {
    pub password: &'a str,
}
