use serde::{Deserialize, Serialize};
use crate::models::ingredients::PostIngredients;

#[derive(Serialize, Queryable)]
pub struct RecipeSteps {
    pub id: i32,
    pub step_number: i32,
    pub instructions: String,
    pub recipe_id: i32,
}

#[derive(Deserialize)]
pub struct PostRecipeSteps {
    step_number: i32,
    instructions: String,
    ingredients: Vec<PostIngredients>
}