use rocket::response::status;
use rocket_contrib::json::Json;
use crate::models::session::{PostSession, Session};
use crate::models::response::GenericResponse;
use crate::Database;
use rocket::config::Environment;
use rocket::http::{Cookie, Cookies};
use crate::models::api_key::LogoutKey;

/// # POST /sessions
/// Creates a new session in the database
///
/// # Example body
/// {
///     "email": "test@gmail.com",
///     "password": "SuperStr0ngPassw0rd!",
/// }
///
#[post("/sessions", data = "<post_session>")]
pub fn post(conn: Database, post_session: Json<PostSession>, mut cookies: Cookies) -> Result<Json<GenericResponse>, status::BadRequest<Json<GenericResponse>>> {
    match post_session.post_session(&*conn) {
        Ok(s) => {
            let env = Environment::active().unwrap();
            let cookie = Cookie::build("session-token", s.token)
                .secure(!env.is_dev())
                .finish();
            // Add cookie to browser
            cookies.add_private(cookie);
            Ok(Json(GenericResponse::default()))
        },
        Err(e) => Err(GenericResponse::new_bad_response(&e.to_string()))
    }
}

/// # DELETE /sessions
/// Deletes a session in the database
#[delete("/sessions")]
pub fn delete(conn: Database, user_logout_key: LogoutKey) -> Result<Json<GenericResponse>, status::BadRequest<Json<GenericResponse>>> {
    match Session::delete(&*conn, user_logout_key.session_id) {
        Ok(t) => Ok(Json(GenericResponse::default())),
        Err(e) => Err(GenericResponse::new_bad_response(&e.to_string()))
    }
}
