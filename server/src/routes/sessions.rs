use crate::models::api_key::{ApiKey, LogoutKey};
use crate::models::response::{GenericResponse, LoginResponse};
use crate::models::session::{LoginStatus, PostSession, Session};
use crate::Database;
use rocket::config::Environment;
use rocket::http::{Cookie, Cookies};
use rocket::response::status;
use rocket_contrib::json::Json;

fn add_private_cookie(session: &Session, mut cookies: Cookies) {
    let env = Environment::active().unwrap();
    let cookie = Cookie::build("session-token", session.token.clone())
        .secure(!env.is_dev())
        .finish();
    // Add cookie to browser
    cookies.add_private(cookie);
}

/// # POST /login
/// Creates a new session in the database
///
/// # Example body
/// ```
/// {
///     "email": "test@gmail.com",
///     "password": "SuperStr0ngPassw0rd!",
/// }
///```
#[post("/login", data = "<post_session>")]
pub fn post(
    conn: Database,
    post_session: Json<PostSession>,
    cookies: Cookies,
) -> Result<Json<LoginResponse>, status::BadRequest<Json<GenericResponse>>> {
    let status = post_session.post_session(&*conn);

    if let Err(e) = status {
        return Err(GenericResponse::new_bad_response(&e.to_string()));
    }

    match status.unwrap() {
        LoginStatus::LoggedIn(s) => {
            add_private_cookie(&s, cookies);
            Ok(Json(LoginResponse::new(true, false)))
        }
        LoginStatus::AlreadyLoggedIn(s) => {
            add_private_cookie(&s, cookies);
            Ok(Json(LoginResponse::new(true, false)))
        }
        LoginStatus::IncorrectPassword => Ok(Json(LoginResponse::new(false, true))),
        LoginStatus::UserDoesNotExist => {
            Err(GenericResponse::new_bad_response("User does not exist."))
        }
    }
}

#[get("/sessions/status")]
pub fn status(_api_key: ApiKey) -> status::NoContent {
    status::NoContent
}

/// # DELETE /api/logout
/// Deletes a session in the database
#[delete("/logout")]
pub fn delete(
    conn: Database,
    user_logout_key: LogoutKey,
) -> Result<Json<GenericResponse>, status::BadRequest<Json<GenericResponse>>> {
    match Session::delete(&*conn, user_logout_key.session_id) {
        Ok(_) => Ok(Json(GenericResponse::default())),
        Err(e) => Err(GenericResponse::new_bad_response(&e.to_string())),
    }
}
