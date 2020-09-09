use crate::schema::recipes;
use serde::{Serialize, Deserialize};
use crate::models::recipe_step::PostRecipeSteps;
use chrono::NaiveDateTime;

#[derive(Serialize, Queryable)]
pub struct Recipe {
    id: i32,
    recipe_name: String,
    recipe_description: Option<String>,
    meal_category: Option<String>,
    preparation_time: Option<String>,
    number_of_servings: Option<i32>,
    calories_per_serving: Option<i32>,
    user_id: Option<i32>,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct PostRecipe {
    recipe_name: String,
    recipe_description: String,
    meal_category: String,
    preparation_time: String,
    number_of_servings: i32,
    calories_per_serving: i32,
    recipe_steps: Vec<PostRecipeSteps>
}