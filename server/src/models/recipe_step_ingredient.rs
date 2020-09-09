use crate::cust_error::Result;
use crate::schema::recipe_step_ingredients;
use diesel::{insert_into, PgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Queryable)]
pub struct RecipeStepIngredient {
    pub id: i32,
    pub step_id: i32,
    pub ingredient_id: i32,
}

impl RecipeStepIngredient {
    pub fn create(
        conn: &PgConnection,
        insert: &InsertRecipeStepIngredient,
    ) -> Result<RecipeStepIngredient> {
        use crate::schema::recipe_step_ingredients::dsl::*;

        Ok(insert_into(recipe_step_ingredients)
            .values(insert)
            .get_result(conn)
            .expect("Failed to insert recipe_step_ingredients"))
    }
}

#[derive(Insertable)]
#[table_name = "recipe_step_ingredients"]
pub struct InsertRecipeStepIngredient {
    pub step_id: i32,
    pub ingredient_id: i32,
}

impl InsertRecipeStepIngredient {
    pub fn new(
        post_step_ingredient: &PostRecipeStepIngredient,
        id: i32,
    ) -> InsertRecipeStepIngredient {
        InsertRecipeStepIngredient {
            step_id: id,
            ingredient_id: post_step_ingredient.ingredient_id,
        }
    }
}

#[derive(Deserialize)]
pub struct PostRecipeStepIngredient {
    pub ingredient_id: i32,
}

impl PostRecipeStepIngredient {
    pub fn create_recipe_step_ingredient(
        &self,
        conn: &PgConnection,
        step_id: i32,
    ) -> Result<RecipeStepIngredient> {
        let insert = InsertRecipeStepIngredient::new(self, step_id);

        RecipeStepIngredient::create(conn, &insert)
    }
}
