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
        status::BadRequest(Some(Json(gr)))
    }

    pub fn new_not_found(message: &str) -> status::NotFound<Json<GenericResponse>> {
        let gr = GenericResponse {
            message: message.to_owned(),
            ok: false,
        };

        status::NotFound(Json(gr))
    }
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub logged_in: bool,
    pub password_is_incorrect: bool,
}

impl LoginResponse {
    pub fn new(logged_in: bool, password_is_incorrect: bool) -> LoginResponse {
        LoginResponse {
            logged_in,
            password_is_incorrect,
        }
    }
}
