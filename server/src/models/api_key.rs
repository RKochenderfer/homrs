use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;

use crate::models::session::Session;
use crate::Database;

#[derive(Debug)]
pub struct ApiKey {
    pub user_id: i32,
    pub session_token: String,
}

#[derive(Debug)]
pub enum ApiKeyError {
    Invalid,
    WrongRole,
    Missing,
    BadCount,
    DatabaseError,
    Expired,
}

impl ApiKey {
    pub fn new(user_id: i32, session_token: String) -> Self {
        ApiKey {
            user_id,
            session_token,
        }
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
            None => return Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)),
        }

        let db = request.guard::<Database>().unwrap();
        // Gets the session by the token
        match Session::get_by_token(&db.0, &token.0) {
            Ok(x) => match x {
                Some(s) => {
                    let now = chrono::Utc::now().naive_utc();
                    // Check if 7 days has passed since last action
                    if now.signed_duration_since(s.last_action).num_days() > 7 {
                        // Delete session
                        if let Err(_) = Session::delete(&db.0, s.id) {
                            Outcome::Failure((
                                Status::InternalServerError,
                                ApiKeyError::DatabaseError,
                            ))
                        } else {
                            // The session token has expired and should be deleted
                            Outcome::Failure((Status::BadRequest, ApiKeyError::Expired))
                        }
                    } else {
                        // Update last used time
                        if let Err(_) = s.update_last_action(&db.0, now) {
                            Outcome::Failure((
                                Status::InternalServerError,
                                ApiKeyError::DatabaseError,
                            ))
                        } else {
                            Outcome::Success(ApiKey::new(s.user_id, token.0))
                        }
                    }
                }
                None => Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)),
            },
            Err(_) => Outcome::Failure((Status::InternalServerError, ApiKeyError::DatabaseError)),
        }
    }
}

pub struct LogoutKey {
    pub session_id: i32,
}

impl LogoutKey {
    pub fn new(session_id: i32) -> LogoutKey {
        LogoutKey { session_id }
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
            Some(cookie) => {
                token = cookie
                    .value()
                    .parse()
                    .ok()
                    .map(|token| Token(token))
                    .unwrap()
            }
            None => return Outcome::Failure((Status::BadRequest, LogoutKeyError::Missing)),
        }

        // Check if session exists
        let db = request.guard::<Database>().unwrap();
        match Session::get_by_token(&db.0, &token.0) {
            Ok(x) => match x {
                Some(s) => Outcome::Success(LogoutKey::new(s.id)),
                None => Outcome::Failure((Status::BadRequest, LogoutKeyError::Missing)),
            },
            Err(_) => {
                Outcome::Failure((Status::InternalServerError, LogoutKeyError::DatabaseError))
            }
        }
    }
}
