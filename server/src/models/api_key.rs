use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;

use crate::models::{
    session::Session
};
use crate::Database;
use crate::schema::sessions::dsl::sessions;


#[derive(Debug)]
pub struct ApiKey {
    pub user_id: i32,
}

#[derive(Debug)]
pub enum ApiKeyError {
    Invalid,
    WrongRole,
    Missing,
    BadCount,
    DatabaseError,
}

impl ApiKey {
    pub fn new(user_id: i32) -> Self {
        ApiKey { user_id }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
    type Error = ApiKeyError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        // Get session token
        struct Token(String);

        let token;
        // Check to make sure the cookie exists
        match request.cookies().get_private("session-token") {
            Some(cookie) => token = cookie.value().parse().map(|token| Token(token)).unwrap(),
            None => return Outcome::Failure((Status::BadRequest, ApiKeyError::Missing))
        }

        let db = request.guard::<Database>().unwrap();
        // Gets the session by the token
        match Session::get_by_token(&db.0, &token.0) {
            Ok(x) => {
                match x {
                    Some(s) => Outcome::Success(ApiKey::new(s.user_id)),
                    None => Outcome::Failure((Status::BadRequest, ApiKeyError::Missing))
                }
            },
            Err(_) => Outcome::Failure((Status::InternalServerError, ApiKeyError::DatabaseError))
        }
    }
}

pub struct LogoutKey {
    pub session_id: i32,
}

impl LogoutKey {
    pub fn new(session_id: i32) -> LogoutKey {
        LogoutKey {
            session_id
        }
    }
}

#[derive(Debug)]
pub enum LogoutKeyError {
    Invalid,
    WrongRole,
    Missing,
    BadCount,
    DatabaseError,
}

impl<'a, 'r> FromRequest<'a, 'r> for LogoutKey {
    type Error = LogoutKeyError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        // Get session token
        struct Token(String);

        let check_cookie = request.cookies().get_private("session-token");
        let token;

        // Check if cookie exists
        match check_cookie {
            Some(cookie) => token = cookie.value().parse().ok().map(|token| Token(token)).unwrap(),
            None => return Outcome::Failure((Status::BadRequest, LogoutKeyError::Missing))
        }

        // Check if session exists
        let db = request.guard::<Database>().unwrap();
        match Session::get_by_token(&db.0, &token.0) {
            Ok(x) => {
                match x {
                    Some(s) => Outcome::Success(LogoutKey::new(s.id)),
                    None => Outcome::Failure((Status::BadRequest, LogoutKeyError::Missing))
                }
            },
            Err(_) => Outcome::Failure((Status::InternalServerError, LogoutKeyError::DatabaseError))
        }
    }
}
