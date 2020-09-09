use serde::{Serialize};

#[derive(Serialize)]
pub struct RecipeStepIngredient {
    pub id: i32,
    pub step_number: i32,
    pub ingredient_id: i32,
}
