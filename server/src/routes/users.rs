use rocket::response::status;
use rocket_contrib::json::Json;
use crate::models::{
    user::{PostUser, User},
};
use crate::cust_error::{BadRequestPostUser};
use crate::models::response::GenericResponse;
use crate::Database;

#[post("/users", data = "<user>")]
fn post(conn: Database, user: Json<PostUser>) -> Result<Json<User>, status::BadRequest<Json<GenericResponse>>> {
    Ok(Json(User::create_user(&*conn, &user.into_inner())))
}