use serde::{Serialize};

#[derive(Debug, Serialize)]
pub struct GenericResponse<'a> {
    pub message: &'a str,
    pub ok: bool,
}
