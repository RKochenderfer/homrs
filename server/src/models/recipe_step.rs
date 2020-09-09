use serde::{Deserialize};
use crate::models::ingredients::PostIngredients;

#[derive(Deserialize)]
pub struct PostRecipeSteps {
    step_number: u32,
    instructions: String,
    ingredients: Vec<PostIngredients>
}