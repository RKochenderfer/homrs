use rocket::response::status;
use rocket_contrib::json::Json;
use crate::models::session::{PostSession, Session, DeleteSession};
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
        Err(e) => Err(GenericResponse::new_bad_response(&e.to_string()))
    }
}

#[delete("/sessions", data = "<delete_session>")]
pub fn delete(conn: Database, delete_session: Json<DeleteSession>) -> Result<Json<GenericResponse>, status::BadRequest<Json<GenericResponse>>> {
    match delete_session.delete_session(&*conn) {
        Ok(t) => Ok(Json(GenericResponse::new(t))),
        Err(e) => Err(GenericResponse::new_bad_response(&e.to_string()))
    }
}
