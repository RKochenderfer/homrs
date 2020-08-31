use serde::{Serialize};
use rocket::response::status;
use rocket_contrib::json::Json;

#[derive(Debug, Serialize)]
pub struct GenericResponse {
    pub message: String,
    pub ok: bool,
}

impl GenericResponse {
    pub fn default() -> GenericResponse {
        GenericResponse {
            message: "".to_owned(),
            ok: true
        }
    }

    pub fn new(message: &str) -> GenericResponse {
        GenericResponse {
            message: message.to_owned(),
            ok: true
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
