use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
struct PostIngredients {
    name: String,
    description: String,
}

#[derive(Deserialize)]
struct PostRecipeSteps {
    step_number: u32,
    instructions: String,
    ingredients: Vec<PostIngredients>
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