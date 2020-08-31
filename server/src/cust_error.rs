use std::fmt;
use std::convert::TryFrom;
use rocket::response::status;
use crate::models::response::GenericResponse;
use rocket_contrib::json::Json;

type BoxedError = Box<dyn std::error::Error + Send + Sync>;

// A simple type alias so as to DRY.
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug)]
pub struct Error(pub &'static str);

impl Error {
    pub fn boxed(error: &str) -> BoxedError {
        Box::try_from(error).unwrap()
    }
}
