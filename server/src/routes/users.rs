use crate::models::api_key::ApiKey;
use crate::models::response::GenericResponse;
use crate::models::user::{PostUser, User, PutUser};
use crate::Database;
use rocket::response::status;
use rocket_contrib::json::Json;

/// # POST /users
/// Creates a new user in the database
///
/// # Example body
/// {
///     "email": "test@gmail.com",
///     "first_name": "Jane",
///     "last_name": "Doe",
///     "password": "SuperStr0ngPassw0rd!",
///     "user_role": "ADM"
/// }
///
#[post("/users", data = "<user>")]
pub fn post(
    conn: Database,
    user: Json<PostUser>,
) -> Result<Json<User>, status::BadRequest<Json<GenericResponse>>> {
    if let Err(e) = user.check_password_strength() {
        return Err(GenericResponse::new_bad_response(&e.to_string()));
    }

    match User::create(&*conn, &user.into_inner()) {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err(GenericResponse::new_bad_response(&e.to_string())),
    }
}

#[put("/users", data = "<put_user>")]
pub fn put(
    conn: Database,
    put_user: Json<PutUser>,
    api_key: ApiKey
) -> Result<Json<User>, status::BadRequest<Json<GenericResponse>>> {
    match User::update(&*conn, put_user.into_inner(), api_key.user_id) {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err(GenericResponse::new_bad_response(&e.to_string())),
    }
}

#[delete("/users")]
pub fn delete(
    conn: Database,
    api_key: ApiKey,
) -> Result<Json<GenericResponse>, status::BadRequest<Json<GenericResponse>>> {
    match User::delete(&*conn, api_key.user_id, api_key.session_token) {
        Ok(_) => Ok(Json(GenericResponse::default())),
        Err(e) => Err(GenericResponse::new_bad_response(&e.to_string())),
    }
}