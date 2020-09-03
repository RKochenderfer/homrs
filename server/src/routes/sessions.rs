use crate::models::api_key::LogoutKey;
use crate::models::response::{GenericResponse, LoginResponse};
use crate::models::session::{PostSession, Session, LoginStatus};
use crate::Database;
use rocket::config::Environment;
use rocket::http::{Cookie, Cookies};
use rocket::response::status;
use rocket_contrib::json::Json;

/// # POST /sessions
/// Creates a new session in the database
///
/// # Example body
/// {
///     "email": "test@gmail.com",
///     "password": "SuperStr0ngPassw0rd!",
/// }
///
#[post("/login", data = "<post_session>")]
pub fn post(
    conn: Database,
    post_session: Json<PostSession>,
    mut cookies: Cookies,
) -> Result<Json<LoginResponse>, status::BadRequest<Json<GenericResponse>>> {
    let status = post_session.post_session(&*conn);

    if let Err(e) = status {
        return Err(GenericResponse::new_bad_response(&e.to_string()));
    }

    match status.unwrap() {
        LoginStatus::LoggedIn(s) => {
            let env = Environment::active().unwrap();
            let cookie = Cookie::build("session-token", s.token)
                .secure(!env.is_dev())
                .finish();
            // Add cookie to browser
            cookies.add_private(cookie);
            Ok(Json(LoginResponse::new(true, false, false)))
        },
        LoginStatus::AlreadyLoggedIn => {
            Ok(Json(LoginResponse::new(false, true, false)))
        },
        LoginStatus::IncorrectPassword => {
            Ok(Json(LoginResponse::new(false, false, true)))
        },
        LoginStatus::UserDoesNotExist => {Err(GenericResponse::new_bad_response("User does not exist."))},
    }
    // match post_session.post_session(&*conn) {
    //     Ok(s) => {
    //         let env = Environment::active().unwrap();
    //         let cookie = Cookie::build("session-token", s.token)
    //             .secure(!env.is_dev())
    //             .finish();
    //         // Add cookie to browser
    //         cookies.add_private(cookie);
    //         Ok(Json(GenericResponse::default()))
    //     }
    //     Err(e) => Err(GenericResponse::new_bad_response(&e.to_string())),
    // }
}

/// # DELETE /sessions
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
