use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct GenericResponse {
    pub message: String,
    pub ok: bool,
}

impl GenericResponse {
    pub fn default() -> GenericResponse {
        GenericResponse {
            message: "".to_owned(),
            ok: true,
        }
    }

    pub fn new(message: &str) -> GenericResponse {
        GenericResponse {
            message: message.to_owned(),
            ok: true,
        }
    }

    /// Generates a bad request to be returned by rocket
    pub fn new_bad_response(message: &str) -> status::BadRequest<Json<GenericResponse>> {
        let gr = GenericResponse {
            message: message.to_owned(),
            ok: false,
        };
        return status::BadRequest(Some(Json(gr)));
    }
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub is_new_session: bool,
    pub does_session_exists: bool,
    pub password_is_incorrect: bool,
}

impl LoginResponse {
    pub fn new(is_new_session: bool, does_session_exists: bool, password_is_incorrect: bool) -> LoginResponse {
        LoginResponse {
            is_new_session,
            does_session_exists,
            password_is_incorrect,
        }
    }
}
