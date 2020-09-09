use serde::{Deserialize};

#[derive(Deserialize)]
pub struct PostIngredients {
    name: String,
    description: String,
}