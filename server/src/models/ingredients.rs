use serde::{Deserialize, Serialize};

#[derive(Serialize, Queryable)]
pub struct Ingredient {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct PostIngredients {
    name: String,
    description: Option<String>,
}
