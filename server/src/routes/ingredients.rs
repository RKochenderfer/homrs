use crate::Database;
use rocket_contrib::json::Json;
use crate::models::api_key::ApiKey;
use crate::models::response::GenericResponse;
use rocket::response::status;
use crate::models::ingredient::{Ingredient, PostIngredient};

/// # POST /api/ingredients
///
/// # Example body
/// ```
/// {
///     "name": "Spaghetti noodles",
///     "description": "Regular spaghetti noodles"
/// }
/// ```
#[post("/ingredients", data = "<post_ingredient>")]
pub fn post(
    conn: Database,
    post_ingredient: Json<PostIngredient>,
    _api_key: ApiKey
) -> Result<Json<Ingredient>, status::BadRequest<Json<GenericResponse>>> {
    match post_ingredient.create_ingredient(&*conn) {
        Ok(ingredient) => Ok(Json(ingredient)),
        Err(e) => Err(GenericResponse::new_bad_response(&e.to_string())),
    }
}