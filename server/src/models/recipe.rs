use serde::{Serialize, Deserialize};
use crate::models::recipe_step::PostRecipeSteps;

#[derive(Serialize)]
pub struct Recipe {
    id: i32,
    recipe_name: String,
    recipe_description: String,
    meal_category: String, // Typically Breakfast, Lunch, Dinner

}

#[derive(Deserialize)]
pub struct PostRecipe {
    recipe_name: String,
    recipe_description: String,
    meal_category: String,
    preparation_time: String,
    number_of_servings: u32,
    calories_per_serving: u32,
    recipe_steps: Vec<PostRecipeSteps>
}