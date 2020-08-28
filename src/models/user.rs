use crate::schema::users;
use chrono::NaiveDateTime;
use diesel::RunQueryDsl;
use serde::{Deserialize, Serialize};

/// Struct modeling a user in the database
#[derive(Queryable)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password_hash: String,
    pub user_role: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl User {
    pub fn get_by_id(id: i32) {

    }
    pub fn get_by_email(email: &str) {

    }
}

/// The struct used to insert a user into the database
#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub password_hash: &'a str,
    pub user_role: &'a str,
}

#[derive(Serialize)]
pub struct PostUser<'a> {
    pub email: &'a strs,
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub password: &'a str,
    pub user_role: &'a str,
}

impl NewUser {
    fn strong_password() {

    }
}
