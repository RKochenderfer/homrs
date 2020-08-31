use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use diesel::{RunQueryDsl, insert_into, PgConnection};
use diesel::prelude::*;
use crate::schema::sessions;
use crate::cust_error::{Result, Error};
use crate::schema::sessions::dsl::*;
use crate::models::user::{User};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Session {
    pub id: i32,
    pub user_id: i32,
    pub token: String,
    pub last_action: NaiveDateTime,
}

impl Session {
    /// Checks to see if a user already has a logged in session
    pub fn check_exists(conn: &PgConnection, check_user_id: i32) -> Result<bool> {
        let result: Vec<Session> = sessions.filter(user_id.eq(check_user_id))
            .load::<Session>(conn)
            .expect("Error loading user");

        if result.len() > 0 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn get_by_token(conn: &PgConnection, session_token: &str) -> Result<Option<Session>> {
        let mut result: Vec<Session> = sessions.filter(token.eq(session_token))
            .load::<Session>(conn)
            .expect("Error loading user");

        if result.len() > 0 {
            Ok(Some(result.pop().unwrap()))
        } else {
            Ok(None)
        }
    }

    /// Inserts a new session into the database
    pub fn create(conn: &PgConnection, new_user_id: i32) -> Result<Session> {
        use uuid::Uuid;

        let uuid = Uuid::new_v4();
        let insert = InsertSession {
            user_id: new_user_id,
            token: uuid.to_string(),
            last_action: chrono::Utc::now().naive_utc(),
        };

        Ok(insert_into(sessions).values(insert).get_result(conn).expect("Failed to insert session."))
    }

    pub fn delete<'a>(conn: &PgConnection, session_id: i32) -> Result<()> {
        let deleted_count = diesel::delete(sessions.filter(id.eq(session_id))).execute(conn)?;

        if deleted_count > 0 {
            Ok(())
        } else {
            Err(Error::boxed(&format!("Session token {} not found", session_id)))
        }
    }
}

#[derive(Insertable)]
#[table_name = "sessions"]
pub struct InsertSession {
    pub user_id: i32,
    pub token: String,
    pub last_action: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct PostSession<'a> {
    pub email: &'a str,
    pub password: &'a str,
}

impl<'a> PostSession<'a> {
    fn password_valid(&self, user: &User) -> bool {
        argon2::verify_encoded(&user.password_hash, self.password.as_bytes()).unwrap()
    }

    /// Handles posting a new session
    pub fn post_session(&self, conn: &PgConnection) -> Result<Session> {
        // Grab user with given email
        if let Some(user) = User::get_by_email(conn,self.email)? {
            if Session::check_exists(conn, user.id)? {
                return Err(Error::boxed("User is already logged in"))
            }
            match self.password_valid(&user) {
                true => Ok(Session::create(conn, user.id)?),
                false => Err(Error::boxed("Incorrect password"))
            }
        } else {
            Err(Error::boxed("Unable to create user session. User does not exist"))
        }
    }
}
