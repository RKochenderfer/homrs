use crate::Database;
use rocket_contrib::json::Json;
use crate::models::recipe::{PostRecipe, Recipe};
use crate::models::api_key::ApiKey;
use crate::models::response::GenericResponse;
use rocket::response::status;

/// # POST /api/users
/// Creates a new recipe in the database
///
/// # Example body
/// ```
/// {
///     "recipe_name": "Spahetti",
///     "recipe_description": "Plain spaghetti with pasta sauce",
///     "meal_category": "dinner",
///     "preparation_time":  "00:30:30",
///     "calories_per_serving": 220,
///     "recipe_steps": [
///         {
///             "step_number": 1,
///             "instructions": "Boil water to a rapid boil",
///             "ingredients": []
///         },
///         {
///             "step_number": 2,
///             "instructions": "Break pasta noodles and throw into pot. Cook the noodles as instructed on the box."
///             "ingredients": [
///                 {
///                     "ingredient_id": 1
///                 },
///             ]
///         },
///         { ... }
///     ]
/// }
/// ```
#[post("/recipes", data = "<post_recipe>")]
pub fn post(
    conn: Database,
    post_recipe: Json<PostRecipe>,
    api_key: ApiKey
) -> Result<Json<Recipe>, status::BadRequest<Json<GenericResponse>>> {
    match post_recipe.create_recipe(&*conn, api_key.user_id) {
        Ok(recipe) => Ok(Json(recipe)),
        Err(e) => Err(GenericResponse::new_bad_response(&e.to_string()))
    }
}
