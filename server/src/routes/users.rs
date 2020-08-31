use rocket::response::status;
use rocket_contrib::json::Json;
use crate::models::{
    user::{PostUser, User},
};
use crate::models::response::GenericResponse;
use crate::Database;

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
pub fn post(conn: Database, user: Json<PostUser>) -> Result<Json<User>, status::BadRequest<Json<GenericResponse>>> {
    if let Err(e) = user.check_password_strength() {
        return Err(GenericResponse::new_bad_request(&e.to_string()));
    }

    match User::create_user(&*conn, &user.into_inner()) {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err(GenericResponse::new_bad_request(&e.to_string()))
    }
}