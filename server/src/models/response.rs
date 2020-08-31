use serde::{Serialize};
use rocket::response::status;
use rocket_contrib::json::Json;

#[derive(Debug, Serialize)]
pub struct GenericResponse {
    pub message: String,
    pub ok: bool,
}

impl GenericResponse {
    pub fn new_bad_request(message: &str) -> status::BadRequest<Json<GenericResponse>> {
        let gr = GenericResponse {
            message: message.to_owned(),
            ok: false,
        };
        return status::BadRequest(Some(Json(gr)));
    }
}
