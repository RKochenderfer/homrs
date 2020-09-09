use crate::cust_error::Result;
use crate::models::recipe_step_ingredient::PostRecipeStepIngredient;
use crate::schema::recipe_steps;
use diesel::{insert_into, PgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Queryable)]
pub struct RecipeStep {
    pub id: i32,
    pub step_number: i32,
    pub instructions: String,
    pub recipe_id: i32,
}

impl RecipeStep {
    pub fn create(conn: &PgConnection, insert: &InsertRecipeStep) -> Result<RecipeStep> {
        use crate::schema::recipe_steps::dsl::*;

        Ok(insert_into(recipe_steps)
            .values(insert)
            .get_result(conn)
            .expect("Failed to insert recipe_step"))
    }
}

#[derive(Insertable)]
#[table_name = "recipe_steps"]
pub struct InsertRecipeStep {
    pub step_number: i32,
    pub instructions: String,
    pub recipe_id: i32,
}

impl InsertRecipeStep {
    pub fn new(post_recipe_step: &PostRecipeStep, id: i32) -> InsertRecipeStep {
        InsertRecipeStep {
            step_number: post_recipe_step.step_number,
            instructions: post_recipe_step.instructions.clone(),
            recipe_id: id,
        }
    }
}

#[derive(Deserialize)]
pub struct PostRecipeStep {
    pub step_number: i32,
    pub instructions: String,
    pub ingredients: Vec<PostRecipeStepIngredient>,
}

impl PostRecipeStep {
    pub fn create_recipe_step(&self, conn: &PgConnection, recipe_id: i32) -> Result<RecipeStep> {
        let insert = InsertRecipeStep::new(self, recipe_id);

        RecipeStep::create(conn, &insert)
    }
}
