use rocket::response::status;
use rocket_contrib::json::Json;
use crate::models::session::{PostSession, Session};
use crate::models::response::GenericResponse;
use crate::Database;

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
pub fn post(conn: Database, post_session: Json<PostSession>) -> Result<Json<Session>, status::BadRequest<Json<GenericResponse>>> {
    match post_session.post_session(&*conn) {
        Ok(s) => Ok(Json(s)),
        Err(e) => Err(GenericResponse::new_bad_request(&e.to_string()))
    }
}